// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use icu::properties::props::{EnumeratedProperty, NumericValue};
use icu::properties::provider::{PropertyCodePointMap, PropertyEnumNumericValueV1};
use icu_provider::prelude::*;

impl DataProvider<PropertyEnumNumericValueV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyEnumNumericValueV1>, DataError> {
        self.check_req::<PropertyEnumNumericValueV1>(req)?;

        let nv_trie = self
            .get_enumerated_prop(
                core::str::from_utf8(NumericValue::NAME).unwrap(),
                core::str::from_utf8(NumericValue::SHORT_NAME).unwrap(),
            )?
            .build_codepointtrie()?;

        let data_struct = PropertyCodePointMap::CodePointTrie(nv_trie);
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<PropertyEnumNumericValueV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}
