// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::borrow::Borrow;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{any::Any, sync::Arc};

use super::{
    execution_mode_from_children,
    ExecutionPlan
};
use crate::metrics::BaselineMetrics;
use crate::stream::ObservedStream;

use arrow::datatypes::{Field, Schema, SchemaRef};
use arrow::record_batch::RecordBatch;
use datafusion_common::stats::Precision;
use datafusion_common::{exec_err, internal_err, Result};
use datafusion_execution::TaskContext;
use datafusion_expr::{expr, Expr};
use datafusion_physical_expr::{calculate_union, EquivalenceProperties};

use futures::Stream;
use itertools::Itertools;
use log::{debug, trace, warn};
use tokio::macros::support::thread_rng_n;

#[derive(Debug)]
pub struct UserDefinedTableFunction {
    /// Input execution plan
    input: Option<Arc<dyn ExecutionPlan>>,
    /// The schema once the unnest is applied
    schema: SchemaRef,
    /// indices of the list-typed columns in the input schema
    expressions: Vec<Arc<Expr>>,
}
