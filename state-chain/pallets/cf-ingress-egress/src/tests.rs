use crate::{
	mock::*, Call as PalletCall, ChannelAction, ChannelIdCounter, CrossChainMessage,
	DepositChannelLookup, DepositChannelPool, DepositWitness, DisabledEgressAssets,
	Event as PalletEvent, FailedVaultTransfers, FetchOrTransfer, MinimumDeposit, Pallet,
	ScheduledEgressCcm, ScheduledEgressFetchOrTransfer, TargetChainAccount, VaultTransfer,
};
use cf_chains::{
	address::AddressConverter, evm::EvmFetchId, mocks::MockEthereum, CcmChannelMetadata,
	DepositChannel, ExecutexSwapAndCall, SwapOrigin, TransferAssetParams,
};
use cf_primitives::{chains::assets::eth, ChannelId, ForeignChain};
use cf_test_utilities::assert_has_event;
use cf_traits::{
	mocks::{
		address_converter::MockAddressConverter,
		api_call::{MockAllBatch, MockEthEnvironment, MockEthereumApiCall},
		block_height_provider::BlockHeightProvider,
		ccm_handler::{CcmRequest, MockCcmHandler},
	},
	DepositApi, EgressApi, GetBlockHeight,
};
use frame_support::{
	assert_ok,
	traits::{Hooks, OriginTrait},
	weights::Weight,
};
use sp_core::H160;

const ALICE_ETH_ADDRESS: EthereumAddress = H160([100u8; 20]);
const BOB_ETH_ADDRESS: EthereumAddress = H160([101u8; 20]);
const ETH_ETH: eth::Asset = eth::Asset::Eth;
const ETH_FLIP: eth::Asset = eth::Asset::Flip;

#[track_caller]
fn expect_size_of_address_pool(size: usize) {
	assert_eq!(
		DepositChannelPool::<Test>::iter_keys().count(),
		size,
		"Address pool size is incorrect!"
	);
}

#[test]
fn blacklisted_asset_will_not_egress_via_batch_all() {
	new_test_ext().execute_with(|| {
		let asset = ETH_ETH;

		// Cannot egress assets that are blacklisted.
		assert!(DisabledEgressAssets::<Test>::get(asset).is_none());
		assert_ok!(IngressEgress::enable_or_disable_egress(RuntimeOrigin::root(), asset, true));
		assert!(DisabledEgressAssets::<Test>::get(asset).is_some());
		System::assert_last_event(RuntimeEvent::IngressEgress(
			crate::Event::AssetEgressStatusChanged { asset, disabled: true },
		));

		// Eth should be blocked while Flip can be sent
		IngressEgress::schedule_egress(asset, 1_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 1_000, ALICE_ETH_ADDRESS, None);

		IngressEgress::on_finalize(1);

		// The egress has not been sent
		assert_eq!(
			ScheduledEgressFetchOrTransfer::<Test>::get(),
			vec![FetchOrTransfer::<Ethereum>::Transfer {
				asset,
				amount: 1_000,
				destination_address: ALICE_ETH_ADDRESS,
				egress_id: (ForeignChain::Ethereum, 1),
			}]
		);

		// re-enable the asset for Egress
		assert_ok!(IngressEgress::enable_or_disable_egress(RuntimeOrigin::root(), asset, false));
		assert!(DisabledEgressAssets::<Test>::get(asset).is_none());
		System::assert_last_event(RuntimeEvent::IngressEgress(
			crate::Event::AssetEgressStatusChanged { asset, disabled: false },
		));

		IngressEgress::on_finalize(1);

		// The egress should be sent now
		assert!(ScheduledEgressFetchOrTransfer::<Test>::get().is_empty());
	});
}

#[test]
fn blacklisted_asset_will_not_egress_via_ccm() {
	new_test_ext().execute_with(|| {
		let asset = ETH_ETH;
		let gas_budget = 1000u128;
		let ccm = CcmDepositMetadata {
			source_chain: ForeignChain::Ethereum,
			source_address: Some(ForeignChainAddress::Eth([0xcf; 20].into())),
			channel_metadata: CcmChannelMetadata {
				message: vec![0x00, 0x01, 0x02].try_into().unwrap(),
				gas_budget: 1_000,
				cf_parameters: vec![].try_into().unwrap(),
			},
		};

		assert!(DisabledEgressAssets::<Test>::get(asset).is_none());
		assert_ok!(IngressEgress::enable_or_disable_egress(RuntimeOrigin::root(), asset, true));

		// Eth should be blocked while Flip can be sent
		IngressEgress::schedule_egress(
			asset,
			1_000,
			ALICE_ETH_ADDRESS,
			Some((ccm.clone(), gas_budget)),
		);
		IngressEgress::schedule_egress(
			ETH_FLIP,
			1_000,
			ALICE_ETH_ADDRESS,
			Some((ccm.clone(), gas_budget)),
		);

		IngressEgress::on_finalize(1);

		// The egress has not been sent
		assert_eq!(
			ScheduledEgressCcm::<Test>::get(),
			vec![CrossChainMessage {
				egress_id: (ForeignChain::Ethereum, 1),
				asset,
				amount: 1_000,
				destination_address: ALICE_ETH_ADDRESS,
				message: ccm.channel_metadata.message.clone(),
				source_chain: ForeignChain::Ethereum,
				source_address: ccm.source_address.clone(),
				cf_parameters: ccm.channel_metadata.cf_parameters,
				gas_budget,
			}]
		);

		// re-enable the asset for Egress
		assert_ok!(IngressEgress::enable_or_disable_egress(RuntimeOrigin::root(), asset, false));

		IngressEgress::on_finalize(2);

		// The egress should be sent now
		assert!(ScheduledEgressCcm::<Test>::get().is_empty());
	});
}

#[test]
fn can_schedule_swap_egress_to_batch() {
	new_test_ext().execute_with(|| {
		IngressEgress::schedule_egress(ETH_ETH, 1_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_ETH, 2_000, ALICE_ETH_ADDRESS, None);
		System::assert_last_event(RuntimeEvent::IngressEgress(crate::Event::EgressScheduled {
			id: (ForeignChain::Ethereum, 2),
			asset: ETH_ETH,
			amount: 2_000,
			destination_address: ALICE_ETH_ADDRESS,
		}));

		IngressEgress::schedule_egress(ETH_FLIP, 3_000, BOB_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 4_000, BOB_ETH_ADDRESS, None);
		System::assert_last_event(RuntimeEvent::IngressEgress(crate::Event::EgressScheduled {
			id: (ForeignChain::Ethereum, 4),
			asset: ETH_FLIP,
			amount: 4_000,
			destination_address: BOB_ETH_ADDRESS,
		}));

		assert_eq!(
			ScheduledEgressFetchOrTransfer::<Test>::get(),
			vec![
				FetchOrTransfer::<Ethereum>::Transfer {
					asset: ETH_ETH,
					amount: 1_000,
					destination_address: ALICE_ETH_ADDRESS,
					egress_id: (ForeignChain::Ethereum, 1),
				},
				FetchOrTransfer::<Ethereum>::Transfer {
					asset: ETH_ETH,
					amount: 2_000,
					destination_address: ALICE_ETH_ADDRESS,
					egress_id: (ForeignChain::Ethereum, 2),
				},
				FetchOrTransfer::<Ethereum>::Transfer {
					asset: ETH_FLIP,
					amount: 3_000,
					destination_address: BOB_ETH_ADDRESS,
					egress_id: (ForeignChain::Ethereum, 3),
				},
				FetchOrTransfer::<Ethereum>::Transfer {
					asset: ETH_FLIP,
					amount: 4_000,
					destination_address: BOB_ETH_ADDRESS,
					egress_id: (ForeignChain::Ethereum, 4),
				},
			]
		);
	});
}

fn request_address_and_deposit(
	who: ChannelId,
	asset: eth::Asset,
) -> (ChannelId, <Ethereum as Chain>::ChainAccount) {
	let (id, address, ..) = IngressEgress::request_liquidity_deposit_address(who, asset).unwrap();
	let address: <Ethereum as Chain>::ChainAccount = address.try_into().unwrap();
	assert_ok!(IngressEgress::process_single_deposit(
		address,
		asset,
		1_000,
		(),
		Default::default()
	));
	(id, address)
}

#[test]
fn can_schedule_deposit_fetch() {
	new_test_ext().execute_with(|| {
		assert!(ScheduledEgressFetchOrTransfer::<Test>::get().is_empty());

		request_address_and_deposit(1u64, eth::Asset::Eth);
		request_address_and_deposit(2u64, eth::Asset::Eth);
		request_address_and_deposit(3u64, eth::Asset::Flip);

		assert!(matches!(
			&ScheduledEgressFetchOrTransfer::<Test>::get()[..],
			&[
				FetchOrTransfer::<Ethereum>::Fetch { asset: ETH_ETH, .. },
				FetchOrTransfer::<Ethereum>::Fetch { asset: ETH_ETH, .. },
				FetchOrTransfer::<Ethereum>::Fetch { asset: ETH_FLIP, .. },
			]
		));

		assert_has_event::<Test>(RuntimeEvent::IngressEgress(
			crate::Event::DepositFetchesScheduled { channel_id: 1, asset: eth::Asset::Eth },
		));

		request_address_and_deposit(4u64, eth::Asset::Eth);

		assert!(matches!(
			&ScheduledEgressFetchOrTransfer::<Test>::get()[..],
			&[
				FetchOrTransfer::<Ethereum>::Fetch { asset: ETH_ETH, .. },
				FetchOrTransfer::<Ethereum>::Fetch { asset: ETH_ETH, .. },
				FetchOrTransfer::<Ethereum>::Fetch { asset: ETH_FLIP, .. },
				FetchOrTransfer::<Ethereum>::Fetch { asset: ETH_ETH, .. },
			]
		));
	});
}

#[test]
fn on_finalize_can_send_batch_all() {
	new_test_ext().execute_with(|| {
		IngressEgress::schedule_egress(ETH_ETH, 1_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_ETH, 2_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_ETH, 3_000, BOB_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_ETH, 4_000, BOB_ETH_ADDRESS, None);
		request_address_and_deposit(1u64, eth::Asset::Eth);
		request_address_and_deposit(2u64, eth::Asset::Eth);
		request_address_and_deposit(3u64, eth::Asset::Eth);
		request_address_and_deposit(4u64, eth::Asset::Eth);

		IngressEgress::schedule_egress(ETH_FLIP, 5_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 6_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 7_000, BOB_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 8_000, BOB_ETH_ADDRESS, None);
		request_address_and_deposit(5u64, eth::Asset::Flip);

		// Take all scheduled Egress and Broadcast as batch
		IngressEgress::on_finalize(1);

		assert_has_event::<Test>(RuntimeEvent::IngressEgress(
			crate::Event::BatchBroadcastRequested {
				broadcast_id: 1,
				egress_ids: vec![
					(ForeignChain::Ethereum, 1),
					(ForeignChain::Ethereum, 2),
					(ForeignChain::Ethereum, 3),
					(ForeignChain::Ethereum, 4),
					(ForeignChain::Ethereum, 5),
					(ForeignChain::Ethereum, 6),
					(ForeignChain::Ethereum, 7),
					(ForeignChain::Ethereum, 8),
				],
			},
		));

		assert!(ScheduledEgressFetchOrTransfer::<Test>::get().is_empty());
	});
}

#[test]
fn all_batch_apicall_creation_failure_should_rollback_storage() {
	new_test_ext().execute_with(|| {
		IngressEgress::schedule_egress(ETH_ETH, 1_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_ETH, 2_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_ETH, 3_000, BOB_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_ETH, 4_000, BOB_ETH_ADDRESS, None);
		request_address_and_deposit(1u64, eth::Asset::Eth);
		request_address_and_deposit(2u64, eth::Asset::Eth);
		request_address_and_deposit(3u64, eth::Asset::Eth);
		request_address_and_deposit(4u64, eth::Asset::Eth);

		IngressEgress::schedule_egress(ETH_FLIP, 5_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 6_000, ALICE_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 7_000, BOB_ETH_ADDRESS, None);
		IngressEgress::schedule_egress(ETH_FLIP, 8_000, BOB_ETH_ADDRESS, None);
		request_address_and_deposit(5u64, eth::Asset::Flip);

		MockAllBatch::<MockEthEnvironment>::set_success(false);
		request_address_and_deposit(4u64, eth::Asset::Usdc);

		let scheduled_requests = ScheduledEgressFetchOrTransfer::<Test>::get();

		// Try to send the scheduled egresses via Allbatch apicall. Will fail and so should rollback
		// the ScheduledEgressFetchOrTransfer
		IngressEgress::on_finalize(1);

		assert_eq!(ScheduledEgressFetchOrTransfer::<Test>::get(), scheduled_requests);
	});
}

#[test]
fn addresses_are_getting_reused() {
	new_test_ext()
		// Request 2 deposit addresses and deposit to one of them.
		.request_address_and_deposit(&[
			(
				DepositRequest::Liquidity { lp_account: ALICE, asset: eth::Asset::Eth },
				100u32.into(),
			),
			(DepositRequest::Liquidity { lp_account: ALICE, asset: eth::Asset::Eth }, 0u32.into()),
		])
		.inspect_storage(|deposit_details| {
			assert_eq!(ChannelIdCounter::<Test, _>::get(), deposit_details.len() as u64);
		})
		// Simulate broadcast success.
		.then_process_events(|_ctx, event| match event {
			RuntimeEvent::IngressEgress(PalletEvent::BatchBroadcastRequested {
				broadcast_id,
				..
			}) => Some(broadcast_id),
			_ => None,
		})
		.then_execute_at_next_block(|(channels, broadcast_ids)| {
			assert!(broadcast_ids.len() == 1);
			// This would normally be triggered on broadcast success, should finalise the ingress.
			for id in broadcast_ids {
				MockEgressBroadcaster::dispatch_callback(id);
			}
			channels
		})
		.then_execute_at_next_block(|channels| {
			let recycle_block = IngressEgress::expiry_and_recycle_block_height().2;
			BlockHeightProvider::<MockEthereum>::set_block_height(recycle_block);

			channels[0].clone()
		})
		// Check that the used address is now deployed and in the pool of available addresses.
		.inspect_storage(|(_request, channel_id, address)| {
			expect_size_of_address_pool(1);
			// Address 1 is free to use and in the pool of available addresses
			assert_eq!(DepositChannelPool::<Test, _>::get(channel_id).unwrap().address, *address);
		})
		.request_deposit_addresses(&[(DepositRequest::Liquidity {
			lp_account: ALICE,
			asset: eth::Asset::Eth,
		})])
		// The address should have been taken from the pool and the id counter unchanged.
		.inspect_storage(|_| {
			expect_size_of_address_pool(0);
			assert_eq!(ChannelIdCounter::<Test, _>::get(), 2);
		});
}

#[test]
fn proof_address_pool_integrity() {
	new_test_ext().execute_with(|| {
		let channel_details = (0..3)
			.map(|id| request_address_and_deposit(id, eth::Asset::Eth))
			.collect::<Vec<_>>();
		// All addresses in use
		expect_size_of_address_pool(0);
		IngressEgress::on_finalize(1);
		for (_id, address) in channel_details {
			assert_ok!(IngressEgress::finalise_ingress(RuntimeOrigin::root(), vec![address]));
		}
		let recycle_block = IngressEgress::expiry_and_recycle_block_height().2;
		BlockHeightProvider::<MockEthereum>::set_block_height(recycle_block);

		IngressEgress::on_idle(1, Weight::MAX);

		// Expect all addresses to be available
		expect_size_of_address_pool(3);
		request_address_and_deposit(4u64, eth::Asset::Eth);
		// Expect one address to be in use
		expect_size_of_address_pool(2);
	});
}

#[test]
fn create_new_address_while_pool_is_empty() {
	new_test_ext().execute_with(|| {
		let channel_details = (0..2)
			.map(|id| request_address_and_deposit(id, eth::Asset::Eth))
			.collect::<Vec<_>>();
		IngressEgress::on_finalize(1);
		for (_id, address) in channel_details {
			assert_ok!(IngressEgress::finalise_ingress(RuntimeOrigin::root(), vec![address]));
		}
		let recycle_block = IngressEgress::expiry_and_recycle_block_height().2;
		BlockHeightProvider::<MockEthereum>::set_block_height(recycle_block);
		IngressEgress::on_idle(1, Weight::MAX);

		assert_eq!(ChannelIdCounter::<Test>::get(), 2);
		request_address_and_deposit(3u64, eth::Asset::Eth);
		assert_eq!(ChannelIdCounter::<Test>::get(), 2);
		IngressEgress::on_finalize(1);
		assert_eq!(ChannelIdCounter::<Test>::get(), 2);
	});
}

#[test]
fn reused_address_channel_id_matches() {
	new_test_ext().execute_with(|| {
		const CHANNEL_ID: ChannelId = 0;
		let new_channel = DepositChannel::<Ethereum>::generate_new::<
			<Test as crate::Config>::AddressDerivation,
		>(CHANNEL_ID, eth::Asset::Eth)
		.unwrap();
		DepositChannelPool::<Test, _>::insert(CHANNEL_ID, new_channel.clone());
		let (reused_channel_id, reused_address, ..) = IngressEgress::open_channel(
			eth::Asset::Eth,
			ChannelAction::LiquidityProvision { lp_account: 0 },
		)
		.unwrap();
		// The reused details should be the same as before.
		assert_eq!(new_channel.channel_id, reused_channel_id);
		assert_eq!(new_channel.address, reused_address);
	});
}

#[test]
fn can_process_ccm_deposit() {
	new_test_ext().execute_with(|| {
		let from_asset = eth::Asset::Flip;
		let to_asset = Asset::Eth;
		let destination_address = ForeignChainAddress::Eth(Default::default());
		let channel_metadata = CcmChannelMetadata {
			message: vec![0x00, 0x01, 0x02].try_into().unwrap(),
			gas_budget: 1_000,
			cf_parameters: vec![].try_into().unwrap(),
		};
		let ccm = CcmDepositMetadata {
			source_chain: ForeignChain::Ethereum,
			source_address: None,
			channel_metadata: channel_metadata.clone(),
		};
		let amount = 5_000;

		// Register swap deposit with CCM

		let (_, deposit_address, ..) = IngressEgress::request_swap_deposit_address(
			from_asset,
			to_asset,
			destination_address.clone(),
			0,
			1,
			Some(channel_metadata),
		)
		.unwrap();

		let deposit_address: TargetChainAccount<Test, _> = deposit_address.try_into().unwrap();

		assert_eq!(
			DepositChannelLookup::<Test>::get(deposit_address).unwrap().opened_at,
			BlockHeightProvider::<MockEthereum>::get_block_height()
		);

		// Making a deposit should trigger CcmHandler.
		assert_ok!(IngressEgress::process_single_deposit(
			deposit_address,
			from_asset,
			amount,
			(),
			Default::default()
		));
		assert_eq!(
			MockCcmHandler::get_ccm_requests(),
			vec![CcmRequest {
				source_asset: from_asset.into(),
				deposit_amount: amount,
				destination_asset: to_asset,
				destination_address,
				deposit_metadata: ccm,
				origin: SwapOrigin::DepositChannel {
					deposit_address: MockAddressConverter::to_encoded_address(
						deposit_address.into()
					),
					channel_id: 1,
					deposit_block_height: Default::default()
				}
			}]
		);
	});
}

#[test]
fn can_egress_ccm() {
	new_test_ext().execute_with(|| {
		let destination_address: H160 = [0x01; 20].into();
		let destination_asset = eth::Asset::Eth;
		let gas_budget = 1_000u128;
		let ccm = CcmDepositMetadata {
			source_chain: ForeignChain::Ethereum,
			source_address: Some(ForeignChainAddress::Eth([0xcf; 20].into())),
			channel_metadata: CcmChannelMetadata {
				message: vec![0x00, 0x01, 0x02].try_into().unwrap(),
				gas_budget,
				cf_parameters: vec![].try_into().unwrap(),
			}
		};
		let amount = 5_000;
		let egress_id = IngressEgress::schedule_egress(
			destination_asset,
			amount,
			destination_address,
			Some((ccm.clone(), gas_budget))
		);

		assert!(ScheduledEgressFetchOrTransfer::<Test>::get().is_empty());
		assert_eq!(ScheduledEgressCcm::<Test>::get(), vec![
			CrossChainMessage {
				egress_id,
				asset: destination_asset,
				amount,
				destination_address,
				message: ccm.channel_metadata.message.clone(),
				cf_parameters: vec![].try_into().unwrap(),
				source_chain: ForeignChain::Ethereum,
				source_address: Some(ForeignChainAddress::Eth([0xcf; 20].into())),
				gas_budget,
			}
		]);
		System::assert_last_event(RuntimeEvent::IngressEgress(
			crate::Event::<Test>::EgressScheduled {
				id: egress_id,
				asset: destination_asset,
				amount,
				destination_address,
			}
		));

		// Send the scheduled ccm in on_finalize
		IngressEgress::on_finalize(1);

		// Check that the CCM should be egressed
		assert_eq!(MockEgressBroadcaster::get_pending_api_calls(), vec![<MockEthereumApiCall<MockEthEnvironment> as ExecutexSwapAndCall<Ethereum>>::new_unsigned(
			(ForeignChain::Ethereum, 1),
			TransferAssetParams {
				asset: destination_asset,
				amount,
				to: destination_address
			},
			ccm.source_chain,
			ccm.source_address,
			gas_budget,
			ccm.channel_metadata.message.to_vec(),
		).unwrap()]);

		// Storage should be cleared
		assert_eq!(ScheduledEgressCcm::<Test>::decode_len(), Some(0));
	});
}

#[test]
fn multi_deposit_includes_deposit_beyond_recycle_height() {
	const ETH: eth::Asset = eth::Asset::Eth;
	new_test_ext()
		.then_execute_at_next_block(|_| {
			let (_, address, ..) =
				IngressEgress::request_liquidity_deposit_address(ALICE, ETH).unwrap();
			let address: <Ethereum as Chain>::ChainAccount = address.try_into().unwrap();
			let recycles_at = IngressEgress::expiry_and_recycle_block_height().2;
			(address, recycles_at)
		})
		.then_execute_at_next_block(|(address, recycles_at)| {
			BlockHeightProvider::<MockEthereum>::set_block_height(recycles_at);
			address
		})
		.then_execute_at_next_block(|address| {
			let (_, address2, ..) =
				IngressEgress::request_liquidity_deposit_address(ALICE, ETH).unwrap();
			let address2: <Ethereum as Chain>::ChainAccount = address2.try_into().unwrap();
			(address, address2)
		})
		.then_apply_extrinsics(|&(address, address2)| {
			[(
				RuntimeOrigin::root(),
				crate::Call::<Test, _>::process_deposits {
					deposit_witnesses: vec![
						DepositWitness {
							deposit_address: address,
							asset: ETH,
							amount: 1,
							deposit_details: Default::default(),
						},
						DepositWitness {
							deposit_address: address2,
							asset: ETH,
							amount: 1,
							deposit_details: Default::default(),
						},
					],
					// The block height is purely informative.
					block_height: BlockHeightProvider::<MockEthereum>::get_block_height(),
				},
				Ok(()),
			)]
		})
		.then_process_events(|_, event| match event {
			RuntimeEvent::IngressEgress(crate::Event::DepositWitnessRejected { .. }) |
			RuntimeEvent::IngressEgress(crate::Event::DepositReceived { .. }) => Some(event),
			_ => None,
		})
		.inspect_context(|((expected_rejected_address, expected_accepted_address), emitted)| {
			assert_eq!(emitted.len(), 2);
			assert!(emitted.iter().any(|e| matches!(
			e,
			RuntimeEvent::IngressEgress(
				crate::Event::DepositWitnessRejected {
					deposit_witness,
					..
				}) if deposit_witness.deposit_address == *expected_rejected_address
			)),);
			assert!(emitted.iter().any(|e| matches!(
			e,
			RuntimeEvent::IngressEgress(
				crate::Event::DepositReceived {
					deposit_address,
					..
				}) if deposit_address == expected_accepted_address
			)),);
		});
}

#[test]
fn multi_use_deposit_address_different_blocks() {
	const ETH: eth::Asset = eth::Asset::Eth;

	new_test_ext()
		.then_execute_at_next_block(|_| request_address_and_deposit(ALICE, ETH))
		.then_apply_extrinsics(|&(_, deposit_address)| {
			[(
				RuntimeOrigin::root(),
				crate::Call::<Test, _>::process_deposits {
					deposit_witnesses: vec![DepositWitness {
						deposit_address,
						asset: ETH,
						amount: 1,
						deposit_details: Default::default(),
					}],
					// block height is purely informative.
					block_height: BlockHeightProvider::<MockEthereum>::get_block_height(),
				},
				Ok(()),
			)]
		})
		.then_execute_at_next_block(|channel @ (_, deposit_address)| {
			assert_ok!(Pallet::<Test, _>::process_single_deposit(
				deposit_address,
				ETH,
				1,
				(),
				Default::default()
			));
			let recycle_block = IngressEgress::expiry_and_recycle_block_height().2;
			BlockHeightProvider::<MockEthereum>::set_block_height(recycle_block);

			channel
		})
		// The channel should be closed at the next block.
		.then_apply_extrinsics(|&(_, deposit_address)| {
			[(
				RuntimeOrigin::root(),
				crate::Call::<Test, _>::process_deposits {
					deposit_witnesses: vec![DepositWitness {
						deposit_address,
						asset: ETH,
						amount: 1,
						deposit_details: Default::default(),
					}],
					// block height is purely informative.
					block_height: BlockHeightProvider::<MockEthereum>::get_block_height(),
				},
				Ok(()),
			)]
		})
		.then_process_events(|_, event| match event {
			RuntimeEvent::IngressEgress(crate::Event::DepositWitnessRejected {
				deposit_witness,
				..
			}) => Some(deposit_witness.deposit_address),
			_ => None,
		})
		.inspect_context(|((_, expected_address), emitted)| {
			assert_eq!(*emitted, vec![*expected_address]);
		});
}

#[test]
fn multi_use_deposit_same_block() {
	// Use FLIP because ETH doesn't trigger a second fetch.
	const FLIP: eth::Asset = eth::Asset::Flip;
	const DEPOSIT_AMOUNT: <Ethereum as Chain>::ChainAmount = 1_000;
	new_test_ext()
		.request_deposit_addresses(&[DepositRequest::Liquidity { lp_account: ALICE, asset: FLIP }])
		.map_context(|mut ctx| {
			assert!(ctx.len() == 1);
			ctx.pop().unwrap()
		})
		.inspect_storage(|(_, _, deposit_address)| {
			assert!(
				DepositChannelLookup::<Test, _>::get(deposit_address)
					.unwrap()
					.deposit_channel
					.state == cf_chains::evm::DeploymentStatus::Undeployed
			);
		})
		.then_apply_extrinsics(|(request, _, deposit_address)| {
			let asset = request.source_asset();
			[(
				OriginTrait::root(),
				PalletCall::<Test, _>::process_deposits {
					deposit_witnesses: vec![
						DepositWitness {
							deposit_address: *deposit_address,
							asset,
							amount: MinimumDeposit::<Test>::get(asset) + DEPOSIT_AMOUNT,
							deposit_details: Default::default(),
						},
						DepositWitness {
							deposit_address: *deposit_address,
							asset,
							amount: MinimumDeposit::<Test>::get(asset) + DEPOSIT_AMOUNT,
							deposit_details: Default::default(),
						},
					],
					block_height: Default::default(),
				},
				Ok(()),
			)]
		})
		.inspect_storage(|(_, channel_id, deposit_address)| {
			assert_eq!(
				DepositChannelLookup::<Test, _>::get(deposit_address)
					.unwrap()
					.deposit_channel
					.state,
				cf_chains::evm::DeploymentStatus::Pending,
			);
			let scheduled_fetches = ScheduledEgressFetchOrTransfer::<Test, _>::get();
			let pending_api_calls = MockEgressBroadcaster::get_pending_api_calls();
			let pending_callbacks = MockEgressBroadcaster::get_pending_callbacks();
			assert!(scheduled_fetches.len() == 1);
			assert!(pending_api_calls.len() == 1);
			assert!(pending_callbacks.len() == 1);
			assert!(
				matches!(
					scheduled_fetches.last().unwrap(),
					FetchOrTransfer::Fetch {
						asset: FLIP,
						..
					}
				),
				"Expected one pending fetch to still be scheduled for the deposit address, got: {:?}",
				scheduled_fetches
			);
			assert!(
				matches!(
					pending_api_calls.last().unwrap(),
					MockEthereumApiCall::AllBatch(MockAllBatch {
						fetch_params,
						..
					}) if matches!(
						fetch_params.last().unwrap().deposit_fetch_id,
						EvmFetchId::DeployAndFetch(id) if id == *channel_id
					)
				),
				"Expected one AllBatch apicall to be scheduled for address deployment, got {:?}.",
				pending_api_calls
			);
			assert!(matches!(
				pending_callbacks.last().unwrap(),
				RuntimeCall::IngressEgress(PalletCall::finalise_ingress { .. })
			));
		})
		.then_execute_at_next_block(|ctx| {
			MockEgressBroadcaster::dispatch_all_callbacks();
			ctx
		})
		.inspect_storage(|(_, _, deposit_address)| {
			assert_eq!(
				DepositChannelLookup::<Test, _>::get(deposit_address)
					.unwrap()
					.deposit_channel
					.state,
				cf_chains::evm::DeploymentStatus::Deployed
			);
			let scheduled_fetches = ScheduledEgressFetchOrTransfer::<Test, _>::get();
			let pending_api_calls = MockEgressBroadcaster::get_pending_api_calls();
			let pending_callbacks = MockEgressBroadcaster::get_pending_callbacks();
			assert!(scheduled_fetches.is_empty());
			assert!(pending_api_calls.len() == 2);
			assert!(pending_callbacks.len() == 1);
			assert!(
				matches!(
					&pending_api_calls[1],
					MockEthereumApiCall::AllBatch(MockAllBatch {
						fetch_params,
						..
					}) if matches!(
						fetch_params.last().unwrap().deposit_fetch_id,
						EvmFetchId::Fetch(address) if address == *deposit_address
					)
				),
				"Expected a new AllBatch apicall to be scheduled to fetch from a deployed address, got {:?}.",
				pending_api_calls
			);
		});
}

#[test]
fn can_set_minimum_deposit() {
	new_test_ext().execute_with(|| {
		let asset = eth::Asset::Eth;
		let minimum_deposit = 1_500u128;
		assert_eq!(MinimumDeposit::<Test>::get(asset), 0);
		// Set the new minimum deposits
		assert_ok!(IngressEgress::set_minimum_deposit(
			RuntimeOrigin::root(),
			asset,
			minimum_deposit
		));

		assert_eq!(MinimumDeposit::<Test>::get(asset), minimum_deposit);

		System::assert_last_event(RuntimeEvent::IngressEgress(
			crate::Event::<Test>::MinimumDepositSet { asset, minimum_deposit },
		));
	});
}

#[test]
fn deposits_below_minimum_are_rejected() {
	new_test_ext().execute_with(|| {
		let eth = eth::Asset::Eth;
		let flip = eth::Asset::Flip;
		let default_deposit_amount = 1_000;

		// Set minimum deposit
		assert_ok!(IngressEgress::set_minimum_deposit(RuntimeOrigin::root(), eth, 1_500));
		assert_ok!(IngressEgress::set_minimum_deposit(
			RuntimeOrigin::root(),
			flip,
			default_deposit_amount
		));

		// Observe that eth deposit gets rejected.
		let (_, deposit_address) = request_address_and_deposit(0, eth);
		System::assert_last_event(RuntimeEvent::IngressEgress(
			crate::Event::<Test>::DepositIgnored {
				deposit_address,
				asset: eth,
				amount: default_deposit_amount,
				deposit_details: Default::default(),
			},
		));

		// Flip deposit should succeed.
		let (_, deposit_address) = request_address_and_deposit(0, flip);
		System::assert_last_event(RuntimeEvent::IngressEgress(
			crate::Event::<Test>::DepositReceived {
				deposit_address,
				asset: flip,
				amount: default_deposit_amount,
				deposit_details: Default::default(),
			},
		));
	});
}

#[test]
fn handle_pending_deployment() {
	const ETH: eth::Asset = eth::Asset::Eth;
	new_test_ext().execute_with(|| {
		// Initial request.
		let (_, deposit_address) = request_address_and_deposit(ALICE, eth::Asset::Eth);
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 1);
		// Process deposits.
		IngressEgress::on_finalize(1);
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 0);
		// Process deposit again the same address.
		Pallet::<Test, _>::process_single_deposit(deposit_address, ETH, 1, (), Default::default())
			.unwrap();
		// None-pending requests can still be sent
		request_address_and_deposit(1u64, eth::Asset::Eth);
		request_address_and_deposit(2u64, eth::Asset::Eth);
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 3);
		// Process deposit again.
		IngressEgress::on_finalize(1);
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 1);
		// Now finalize the first fetch and deploy the address with that.
		assert_ok!(IngressEgress::finalise_ingress(RuntimeOrigin::root(), vec![deposit_address]));
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 1);
		// Process deposit again amd expect the fetch request to be picked up.
		IngressEgress::on_finalize(1);
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 0);
	});
}

#[test]
fn handle_pending_deployment_same_block() {
	new_test_ext().execute_with(|| {
		// Initial request.
		let (_, deposit_address) = request_address_and_deposit(ALICE, eth::Asset::Eth);
		Pallet::<Test, _>::process_single_deposit(
			deposit_address,
			eth::Asset::Eth,
			1,
			(),
			Default::default(),
		)
		.unwrap();
		// Expect to have two fetch requests.
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 2);
		// Process deposits.
		IngressEgress::on_finalize(1);
		// Expect only one fetch request processed in one block. Note: This not the most performant
		// solution, but also an edge case. Maybe we can improve this in the future.
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 1);
		// Process deposit (again).
		IngressEgress::on_finalize(2);
		// Expect the request still to be in the queue.
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 1);
		// Simulate the finalization of the first fetch request.
		assert_ok!(IngressEgress::finalise_ingress(RuntimeOrigin::root(), vec![deposit_address]));
		// Process deposit (again).
		IngressEgress::on_finalize(3);
		// All fetch requests should be processed.
		assert_eq!(ScheduledEgressFetchOrTransfer::<Test, _>::decode_len().unwrap_or_default(), 0);
	});
}

#[test]
fn channel_reuse_with_different_assets() {
	const ASSET_1: eth::Asset = eth::Asset::Eth;
	const ASSET_2: eth::Asset = eth::Asset::Flip;
	new_test_ext()
		// First, request a deposit address and use it, then close it so it gets recycled.
		.request_address_and_deposit(&[(
			DepositRequest::Liquidity { lp_account: ALICE, asset: ASSET_1 },
			100_000,
		)])
		.map_context(|mut result| result.pop().unwrap())
		.then_execute_at_next_block(|ctx| {
			// Dispatch callbacks to finalise the ingress.
			MockEgressBroadcaster::dispatch_all_callbacks();
			ctx
		})
		.inspect_storage(|(request, _, address)| {
			let asset = request.source_asset();
			assert_eq!(asset, ASSET_1);
			assert!(
				DepositChannelLookup::<Test, _>::get(address).unwrap().deposit_channel.asset ==
					asset
			);
		})
		.then_execute_at_next_block(|(_, channel_id, _)| {
			let recycle_block = IngressEgress::expiry_and_recycle_block_height().2;
			BlockHeightProvider::<MockEthereum>::set_block_height(recycle_block);
			channel_id
		})
		.inspect_storage(|channel_id| {
			assert!(DepositChannelLookup::<Test, _>::get(ALICE_ETH_ADDRESS).is_none());
			assert!(
				DepositChannelPool::<Test, _>::iter_values().next().unwrap().channel_id ==
					*channel_id
			);
		})
		// Request a new address with a different asset.
		.request_deposit_addresses(&[DepositRequest::Liquidity {
			lp_account: ALICE,
			asset: ASSET_2,
		}])
		.map_context(|mut result| result.pop().unwrap())
		// Ensure that the deposit channel's asset is updated.
		.inspect_storage(|(request, _, address)| {
			let asset = request.source_asset();
			assert_eq!(asset, ASSET_2);
			assert!(
				DepositChannelLookup::<Test, _>::get(address).unwrap().deposit_channel.asset ==
					asset
			);
		});
}

/// This is the sequence we're testing.
/// 1. Request deposit address
/// 2. Deposit to address when it's almost expired
/// 3. The channel is expired
/// 4. We need to finalise the ingress, by fetching
/// 5. The fetch should succeed.
#[test]
fn ingress_finalisation_succeeds_after_channel_expired_but_not_recycled() {
	new_test_ext().execute_with(|| {
		assert!(ScheduledEgressFetchOrTransfer::<Test>::get().is_empty(), "Is empty after genesis");

		request_address_and_deposit(ALICE, eth::Asset::Eth);

		// Because we're only *expiring* and not recycling, we should still be able to fetch.
		let expiry_block = IngressEgress::expiry_and_recycle_block_height().1;
		BlockHeightProvider::<MockEthereum>::set_block_height(expiry_block);

		IngressEgress::on_idle(1, Weight::MAX);

		IngressEgress::on_finalize(1);

		assert!(ScheduledEgressFetchOrTransfer::<Test>::get().is_empty(),);
	});
}

#[test]
fn can_store_failed_vault_transfers() {
	new_test_ext().execute_with(|| {
		let vault_transfer = VaultTransfer::<Ethereum> {
			asset: eth::Asset::Eth,
			amount: 1_000_000u128,
			destination_address: [0xcf; 20].into(),
		};

		assert_ok!(IngressEgress::vault_transfer_failed(
			RuntimeOrigin::root(),
			vault_transfer.asset,
			vault_transfer.amount,
			vault_transfer.destination_address,
		));

		assert_has_event::<Test>(RuntimeEvent::IngressEgress(PalletEvent::VaultTransferFailed {
			asset: vault_transfer.asset,
			amount: vault_transfer.amount,
			destination_address: vault_transfer.destination_address,
		}));
		assert_eq!(FailedVaultTransfers::<Test>::get(), vec![vault_transfer]);
	});
}

#[test]
fn basic_balance_tracking() {
	const ETH_DEPOSIT_AMOUNT: u128 = 1_000;
	const FLIP_DEPOSIT_AMOUNT: u128 = 2_000;
	const USDC_DEPOSIT_AMOUNT: u128 = 3_000;

	new_test_ext()
		.check_deposit_balances(&[
			(eth::Asset::Eth, 0),
			(eth::Asset::Flip, 0),
			(eth::Asset::Usdc, 0),
		])
		.request_address_and_deposit(&[(
			DepositRequest::Liquidity { lp_account: ALICE, asset: eth::Asset::Eth },
			ETH_DEPOSIT_AMOUNT,
		)])
		.check_deposit_balances(&[
			(eth::Asset::Eth, ETH_DEPOSIT_AMOUNT),
			(eth::Asset::Flip, 0),
			(eth::Asset::Usdc, 0),
		])
		.request_address_and_deposit(&[(
			DepositRequest::Liquidity { lp_account: ALICE, asset: eth::Asset::Flip },
			FLIP_DEPOSIT_AMOUNT,
		)])
		.check_deposit_balances(&[
			(eth::Asset::Eth, ETH_DEPOSIT_AMOUNT),
			(eth::Asset::Flip, FLIP_DEPOSIT_AMOUNT),
			(eth::Asset::Usdc, 0),
		])
		.request_address_and_deposit(&[(
			DepositRequest::Liquidity { lp_account: ALICE, asset: eth::Asset::Usdc },
			USDC_DEPOSIT_AMOUNT,
		)])
		.check_deposit_balances(&[
			(eth::Asset::Eth, ETH_DEPOSIT_AMOUNT),
			(eth::Asset::Flip, FLIP_DEPOSIT_AMOUNT),
			(eth::Asset::Usdc, USDC_DEPOSIT_AMOUNT),
		])
		.request_address_and_deposit(&[(
			DepositRequest::Liquidity { lp_account: ALICE, asset: eth::Asset::Eth },
			ETH_DEPOSIT_AMOUNT,
		)])
		.check_deposit_balances(&[
			(eth::Asset::Eth, ETH_DEPOSIT_AMOUNT * 2),
			(eth::Asset::Flip, FLIP_DEPOSIT_AMOUNT),
			(eth::Asset::Usdc, USDC_DEPOSIT_AMOUNT),
		])
		.request_address_and_deposit(&[(
			DepositRequest::SimpleSwap {
				source_asset: eth::Asset::Eth,
				destination_asset: eth::Asset::Flip,
				destination_address: ForeignChainAddress::Eth(Default::default()),
			},
			ETH_DEPOSIT_AMOUNT,
		)])
		.check_deposit_balances(&[
			(eth::Asset::Eth, ETH_DEPOSIT_AMOUNT * 3),
			(eth::Asset::Flip, FLIP_DEPOSIT_AMOUNT - ETH_DEPOSIT_AMOUNT),
			(eth::Asset::Usdc, USDC_DEPOSIT_AMOUNT),
		]);
}

#[test]
fn test_default_empty_amounts() {
	let mut channel_recycle_blocks = Default::default();
	let can_recycle = IngressEgress::can_and_cannot_recycle(&mut channel_recycle_blocks, 0, 0);

	assert_eq!(can_recycle, vec![]);
	assert_eq!(channel_recycle_blocks, vec![]);
}

#[test]
fn test_cannot_recycle_if_block_number_less_than_current_height() {
	let maximum_recyclable_number = 2;
	let mut channel_recycle_blocks =
		(1u64..5).map(|i| (i, H160::from([i as u8; 20]))).collect::<Vec<_>>();
	let current_block_height = 3;

	let can_recycle = IngressEgress::can_and_cannot_recycle(
		&mut channel_recycle_blocks,
		maximum_recyclable_number,
		current_block_height,
	);

	assert_eq!(can_recycle, vec![H160::from([1u8; 20]), H160::from([2; 20])]);
	assert_eq!(
		channel_recycle_blocks,
		vec![(3, H160::from([3u8; 20])), (4, H160::from([4u8; 20]))]
	);
}

// Same test as above, but lower maximum recyclable number
#[test]
fn test_can_only_recycle_up_to_max_amount() {
	let maximum_recyclable_number = 1;
	let mut channel_recycle_blocks =
		(1u64..5).map(|i| (i, H160::from([i as u8; 20]))).collect::<Vec<_>>();
	let current_block_height = 3;

	let can_recycle = IngressEgress::can_and_cannot_recycle(
		&mut channel_recycle_blocks,
		maximum_recyclable_number,
		current_block_height,
	);

	assert_eq!(can_recycle, vec![H160::from([1u8; 20])]);
	assert_eq!(
		channel_recycle_blocks,
		vec![(2, H160::from([2; 20])), (3, H160::from([3u8; 20])), (4, H160::from([4u8; 20]))]
	);
}

#[test]
fn none_can_be_recycled_due_to_low_block_number() {
	let maximum_recyclable_number = 4;
	let mut channel_recycle_blocks =
		(1u64..5).map(|i| (i, H160::from([i as u8; 20]))).collect::<Vec<_>>();
	let current_block_height = 0;

	let can_recycle = IngressEgress::can_and_cannot_recycle(
		&mut channel_recycle_blocks,
		maximum_recyclable_number,
		current_block_height,
	);

	assert!(can_recycle.is_empty());
	assert_eq!(
		channel_recycle_blocks,
		vec![
			(1, H160::from([1u8; 20])),
			(2, H160::from([2; 20])),
			(3, H160::from([3; 20])),
			(4, H160::from([4; 20]))
		]
	);
}

#[test]
fn all_can_be_recycled() {
	let maximum_recyclable_number = 4;
	let mut channel_recycle_blocks =
		(1u64..5).map(|i| (i, H160::from([i as u8; 20]))).collect::<Vec<_>>();
	let current_block_height = 4;

	let can_recycle = IngressEgress::can_and_cannot_recycle(
		&mut channel_recycle_blocks,
		maximum_recyclable_number,
		current_block_height,
	);

	assert_eq!(
		can_recycle,
		vec![H160::from([1u8; 20]), H160::from([2; 20]), H160::from([3; 20]), H160::from([4; 20])]
	);
	assert!(channel_recycle_blocks.is_empty());
}
