# structured-instructions

No more pain when parsing instructions. Simply

```rust
use substreams_solana_structured_instructions::get_structured_instructions

let structured_instructions = get_structured_instructions(confirmed_transaction)
```

et voilà! `structured_instructions` here is represented by a `Vec<StructuredInstruction>` where

```rust
struct StructuredInstruction {
    inner_instructions: Vec<StructuredInstruction>,
    ...
}
```

so that you can quickly query the programs that were directly invoked by another without jumping through hoops. See an example with [raydium-substream](https://github.com/0xpapercut/raydium-substream).
