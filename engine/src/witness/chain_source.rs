pub mod btc_source;
pub mod dot_source;
pub mod eth_source;
pub mod extension;
pub mod lag_safety;
pub mod shared;
pub mod strictly_monotonic;
pub mod then;

use std::pin::Pin;

use futures_core::Stream;

pub mod aliases {
	use std::iter::Step;

	macro_rules! define_trait_alias {
		(pub trait $name:ident: $($traits:tt)+) => {
			pub trait $name: $($traits)+ {}
			impl<T: $($traits)+> $name for T {}
		}
	}

	define_trait_alias!(pub trait Index: Step + PartialEq + Eq + PartialOrd + Ord + Clone + Copy + Send + Sync + Unpin + 'static);
	define_trait_alias!(pub trait Hash: PartialEq + Eq + Clone + Copy + Send + Sync + Unpin + 'static);
	define_trait_alias!(pub trait Data: Send + Sync + Unpin + 'static);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header<Index, Hash, Data> {
	pub index: Index,
	pub hash: Hash,
	pub parent_hash: Option<Hash>,
	pub data: Data,
}

#[async_trait::async_trait]
pub trait ChainSource: Send + Sync {
	type Index: aliases::Index;
	type Hash: aliases::Hash;
	type Data: aliases::Data;

	type Client: ChainClient<Index = Self::Index, Hash = Self::Hash, Data = Self::Data>;

	async fn stream_and_client(
		&self,
	) -> (BoxChainStream<'_, Self::Index, Self::Hash, Self::Data>, Self::Client);
}

#[async_trait::async_trait]
pub trait ChainClient: Send + Sync {
	type Index: aliases::Index;
	type Hash: aliases::Hash;
	type Data: aliases::Data;

	async fn header_at_index(
		&self,
		index: Self::Index,
	) -> Header<Self::Index, Self::Hash, Self::Data>;

	fn into_box<'a>(self) -> BoxChainClient<'a, Self::Index, Self::Hash, Self::Data>
	where
		Self: 'a + Sized,
	{
		Box::new(self)
	}
}
pub type BoxChainClient<'a, Index, Hash, Data> =
	Box<dyn ChainClient<Index = Index, Hash = Hash, Data = Data> + 'a>;

pub trait ChainStream: Stream<Item = Header<Self::Index, Self::Hash, Self::Data>> + Send {
	type Index: aliases::Index;
	type Hash: aliases::Hash;
	type Data: aliases::Data;

	fn into_box<'a>(self) -> BoxChainStream<'a, Self::Index, Self::Hash, Self::Data>
	where
		Self: 'a + Sized,
	{
		Box::pin(self)
	}
}
impl<
		Index: aliases::Index,
		Hash: aliases::Hash,
		Data: aliases::Data,
		T: Stream<Item = Header<Index, Hash, Data>> + Send,
	> ChainStream for T
{
	type Index = Index;
	type Hash = Hash;
	type Data = Data;
}
pub type BoxChainStream<'a, Index, Hash, Data> = Pin<
	Box<
		dyn ChainStream<Index = Index, Hash = Hash, Data = Data, Item = Header<Index, Hash, Data>>
			+ Send
			+ 'a,
	>,
>;