use crate::stats::get_stats;
use alloy::{primitives::Address, providers::ProviderBuilder, sol};
use UserStats::UserStatsInstance;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UserStats,
    "src/stat_handler/UserStats.json"
);

pub struct UserStatsHandler {
    contract: UserStatsInstance<
        (),
        alloy::providers::fillers::FillProvider<
            alloy::providers::fillers::JoinFill<
                alloy::providers::Identity,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::GasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::BlobGasFiller,
                        alloy::providers::fillers::JoinFill<
                            alloy::providers::fillers::NonceFiller,
                            alloy::providers::fillers::ChainIdFiller,
                        >,
                    >,
                >,
            >,
            alloy::providers::RootProvider,
            alloy::network::Ethereum,
        >,
    >,
}

impl UserStatsHandler {
    pub fn new(rpc_url: String, address: Address) -> Self {
        let rpc_url = rpc_url.parse().unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url);
        let contract = UserStats::new(address, provider);
        Self { contract }
    }

    pub async fn update_user_stats(&self) -> Result<(), anyhow::Error> {
        let stats = get_stats();
        self.contract.updateStats(stats.to_string()).call().await?;
        Ok(())
    }
}
