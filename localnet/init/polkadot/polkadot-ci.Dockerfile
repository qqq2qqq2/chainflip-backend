ARG TAG=v0.9.36
FROM ghcr.io/chainflip-io/chainflip-private-polkadot/polkadot:${TAG}

COPY chainspec.json /polkadot

ENTRYPOINT polkadot \
           --alice \
           --blocks-pruning=archive \
           --chain=/polkadot/chainspec.json \
           --force-authoring \
           --name=PolkaDocker \
           --rpc-cors=all \
           --rpc-external \
           --rpc-methods=unsafe \
           --state-pruning=archive \
           --validator \
           --ws-external