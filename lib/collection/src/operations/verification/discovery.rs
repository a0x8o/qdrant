use segment::types::{Filter, SearchParams, StrictModeConfig};

use super::StrictModeVerification;
use crate::collection::Collection;
use crate::operations::types::{CollectionResult, DiscoverRequestBatch, DiscoverRequestInternal};

impl StrictModeVerification for DiscoverRequestInternal {
    fn query_limit(&self) -> Option<usize> {
        Some(self.limit)
    }

    fn indexed_filter_read(&self) -> Option<&Filter> {
        self.filter.as_ref()
    }

    fn indexed_filter_write(&self) -> Option<&Filter> {
        None
    }

    fn request_exact(&self) -> Option<bool> {
        None
    }

    fn request_search_params(&self) -> Option<&SearchParams> {
        self.params.as_ref()
    }
}

impl StrictModeVerification for DiscoverRequestBatch {
    async fn check_strict_mode(
        &self,
        collection: &Collection,
        strict_mode_config: &StrictModeConfig,
    ) -> CollectionResult<()> {
        for i in self.searches.iter() {
            i.discover_request
                .check_strict_mode(collection, strict_mode_config)
                .await?;
        }

        Ok(())
    }

    fn query_limit(&self) -> Option<usize> {
        None
    }

    fn indexed_filter_read(&self) -> Option<&Filter> {
        None
    }

    fn indexed_filter_write(&self) -> Option<&Filter> {
        None
    }

    fn request_exact(&self) -> Option<bool> {
        None
    }

    fn request_search_params(&self) -> Option<&SearchParams> {
        None
    }
}
