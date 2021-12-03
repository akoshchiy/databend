// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::DataSchemaRef;
use common_meta_types::MetaId;

use crate::Expression;
use crate::PlanNode;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum InputSource {
    SelectPlan(Box<PlanNode>),
    Expressions(Vec<Vec<Expression>>),
    StreamingWithFormat(String),
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct InsertIntoPlan {
    pub database_name: String,
    pub table_name: String,
    pub table_id: MetaId,
    pub schema: DataSchemaRef,
    pub overwrite: bool,
    pub source: InputSource,
}

impl PartialEq for InsertIntoPlan {
    fn eq(&self, other: &Self) -> bool {
        self.database_name == other.database_name
            && self.table_name == other.table_name
            && self.schema == other.schema
    }
}

impl InsertIntoPlan {
    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }

    pub fn insert_select(
        db: String,
        table: String,
        table_meta_id: MetaId,
        schema: DataSchemaRef,
        overwrite: bool,
        select_plan: PlanNode,
    ) -> InsertIntoPlan {
        InsertIntoPlan {
            database_name: db,
            table_name: table,
            table_id: table_meta_id,
            schema,
            overwrite,
            source: InputSource::SelectPlan(Box::new(select_plan)),
        }
    }

    pub fn insert_values(
        db: String,
        table: String,
        table_meta_id: MetaId,
        schema: DataSchemaRef,
        overwrite: bool,
        values: Vec<Vec<Expression>>,
    ) -> InsertIntoPlan {
        InsertIntoPlan {
            database_name: db,
            table_name: table,
            table_id: table_meta_id,
            schema,
            overwrite,
            source: InputSource::Expressions(values),
        }
    }

    pub fn insert_without_source(
        db: String,
        table: String,
        table_meta_id: MetaId,
        schema: DataSchemaRef,
        overwrite: bool,
        format: String,
    ) -> InsertIntoPlan {
        InsertIntoPlan {
            database_name: db,
            table_name: table,
            table_id: table_meta_id,
            schema,
            overwrite,
            source: InputSource::StreamingWithFormat(format),
        }
    }
}
