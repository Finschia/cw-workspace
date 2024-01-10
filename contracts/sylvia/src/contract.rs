use cosmwasm_std::Response;
use sylvia::types::InstantiateCtx;
use sylvia::{contract, entry_points};

use crate::error::ContractError;

// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus
//
// ## Example
//
// ```rust
// pub struct CounterContract<'a> {
//     counter: Item<'a, int>,
// }
// ```

pub struct {{project-name|pascal_case}}Contract {}

#[entry_points]
#[contract]
#[error(crate::error::ContractError)]
impl {{project-name|pascal_case}}Contract {
    pub const fn new() -> Self {
        Self {}
    }

    // Instantiate msg constuction and handler.
    //
    // All arguments following `ctx` will be used as fields in the instantiate message.
    //
    // ## Example
    //
    // ```rust
    //
    // #[msg(instantiate)]
    // pub fn instantiate(&self, ctx: InstantiateCtx, count: u32) -> StdResult<Response> {
    //     self.count.save(ctx.deps.storage, &count)?;
    //     Ok(Response::default())
    // }
    // ```

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx) -> Result<Response, ContractError> {
        Ok(Response::default())
    }

    // Query msg constuction and handler.
    //
    // All arguments following `ctx` will be used as fields in the query message.
    //
    // ## Example
    //
    // ```rust
    // #[msg(query)]
    // pub fn count(&self, ctx: QueryCtx) -> StdResult<CountResponse> {
    //     let count = self.count.load(ctx.deps.storage)?;
    //     Ok(CountResponse { count })
    // }
    // ```
    // For response type, see `./responses.rs`

    // Execute msg constuction and handler.
    //
    // All arguments following `ctx` will be used as fields in the execute message.
    //
    // ## Example
    //
    // ```rust
    // #[msg(exec)]
    // pub fn increment_count(&self, ctx: ExecCtx, ) -> StdResult<Response> {
    //     self.count
    //         .update(ctx.deps.storage, |count| -> StdResult<u32> {
    //             Ok(count + 1)
    //         })?;
    //     Ok(Response::default())
    // }
    // ```
}
