# WARP

**WARP** provides a common format for transferring and applying function information across binary analysis tools.

## WARP Integrations

### Binary Ninja

WARP integration is available as an [open source](https://github.com/Vector35/binaryninja-api/tree/dev/plugins/warp) first-party plugin for [Binary Ninja] and as such ships by default.

## Function Identification

Function identification is the main way to interact with **WARP**, allowing tooling to utilize **WARP**'s dataset to identify
common functions within any binary efficiently and accurately.

### Integration Requirements

To integrate with **WARP** function matching, you must be able to:

1. Disassemble instructions
2. Identify basic blocks that make up a function
3. Identify register groups with implicit extend operation
4. Identify relocatable instructions (see [What is considered a relocatable instruction?](#what-is-considered-a-relocatable-instruction))

### Creating a Function GUID

The function GUID is the UUIDv5 of the basic block GUIDs (sorted highest to lowest start address) that make up the function.

#### Example

Given the following sorted basic blocks:

1. `036cccf0-8239-5b84-a811-60efc2d7eeb0`
2. `3ed5c023-658d-5511-9710-40814f31af50`
3. `8a076c92-0ba0-540d-b724-7fd5838da9df`

The function GUID will be `7a55be03-76b7-5cb5-bae9-4edcf47795ac`.

##### Example Code

```py
import uuid

def uuid5(namespace, name_bytes):
  """Generate a UUID from the SHA-1 hash of a namespace UUID and a name bytes."""
  from hashlib import sha1
  hash = sha1(namespace.bytes + name_bytes).digest()
  return uuid.UUID(bytes=hash[:16], version=5)

function_namespace = uuid.UUID('0192a179-61ac-7cef-88ed-012296e9492f')
bb1 = uuid.UUID("036cccf0-8239-5b84-a811-60efc2d7eeb0")
bb2 = uuid.UUID("3ed5c023-658d-5511-9710-40814f31af50")
bb3 = uuid.UUID("8a076c92-0ba0-540d-b724-7fd5838da9df")
function = uuid5(function_namespace, bb1.bytes + bb2.bytes + bb3.bytes)
```

#### What is the UUIDv5 namespace?

The namespace for Function GUIDs is `0192a179-61ac-7cef-88ed-012296e9492f`.

### Creating a Basic Block GUID

The basic block GUID is the UUIDv5 of the byte sequence of the instructions (sorted in execution order) with the following properties:

1. Zero out all relocatable instructions.
2. Exclude all NOP instructions.
3. Exclude all instructions that set a register to itself if they are effectively NOPs.

#### When are instructions that set a register to itself removed?

To support hot-patching, we must remove them as they can be injected by the compiler at the start of a function (see: [1] and [2]).
This does not affect the accuracy of the function GUID as they are only removed when the instruction is a NOP:

- Register groups with no implicit extension will be removed (see: [3] (under 3.4.1.1))

For the `x86_64` architecture this means `mov edi, edi` will _not_ be removed, but it _will_ be removed for the `x86` architecture.

#### What is considered a relocatable instruction?

An instruction with an operand that is used as a _constant_ pointer to a mapped region.

 - For the `x86` architecture the instruction `e8b55b0100` (or `call 0x15bba`) would be zeroed.

An instruction which is used to calculate a _constant_ pointer to a mapped region, with a _constant_ offset.

- For the `aarch64` architecture the instruction `21403c91` (or `add x1, x1, #0xf10`) would be zeroed if the incoming `x1` was a pointer into a mapped region.

#### What is the UUIDv5 namespace?

The namespace for Basic Block GUIDs is `0192a178-7a5f-7936-8653-3cbaa7d6afe7`.

### Constraints

Constraints allow us to further disambiguate between functions with the same GUID; when creating the functions, we retrieve extra information
that is consistent between versions of the same function, some examples are:

- Called functions
- Caller functions
- Adjacent functions

Each extra piece of information is referred to as a "constraint" that can be used to further reduce the number of matches for a given function GUID.

#### Creating a Constraint

Constraints are made up of a GUID and optionally, a matching offset. Adding a matching offset is preferred to give locality to the constraints,
for example, if you have a function `A` which calls into function `B` that is one constraint, but if the function `B` is also adjacent to function `A`
without a matching offset the two constraints may be merged into a single one, reducing the number of matching constraints.

- The adjacent function `B` as a constraint: `(9F188A12-3EA1-477D-B368-361936EEA213, -30)`
- The call to function `B` as a constraint: `(9F188A12-3EA1-477D-B368-361936EEA213, 48)`

#### Creating a Constraint GUID

The constraint GUID is the UUIDv5 of the relevant bytes that would be computable at creation time and lookup time.

##### What is the UUIDv5 namespace?

The namespace for Constraint GUIDs is `019701f3-e89c-7afa-9181-371a5e98a576`.

##### Why don't we require matching on constraints for trivial functions?

The decision to match on constraints is left to the user. While requiring constraint matching for functions
from all datasets can reduce false positives, it may not always be necessary. For example, when transferring functions
from one version of a binary to another version of the same binary, not matching on constraints for trivial functions
might be acceptable.

## Comparison of Function Recognition Tools

### WARP vs FLIRT

The main difference between **WARP** and **FLIRT** is the approach to identification.

#### Function Identification

- **WARP** the function identification is described [here](#function-identification).
- **FLIRT** uses an incomplete function byte sequence with a mask where there is a single function entry (see: [IDA FLIRT Documentation] for a full description).

What this means in practice is **WARP** will have fewer false positives based solely off the initial function identification.
When the returned set of functions is greater than one, we can use the list of [Function Constraints](#function-constraints) to select the best possible match.
However, that comes at the cost of requiring a computed GUID to be created whenever the lookup is requested and that the function GUID is _**always**_ the same.

### WARP vs SigKit

Because WARP is a replacement for SigKit it makes sense to not only talk about the function identification approach, but also the integration with [Binary Ninja].

#### SigKit Function Identification

SigKit's function identification is similar to FLIRT so to not repeat what is said above, see [here](#function-identification).

One difference to point out is SigKit relies on relocations during signature generation. Because of this, firmware or other types of binaries lacking relocations will likely fail to mask off the required instructions.

#### Binary Ninja Integration

The two main processes that exist for both SigKit and WARP integration with Binary Ninja are the function lookup process and the signature generation process. 

##### Function lookup

SigKit's function lookup process is integrated as a core component to Binary Ninja as such it is not open source, however, the process is described [here](https://binary.ninja/2020/03/11/signature-libraries.html).

What this means is **WARP** unlike SigKit can identify a greater number of smaller functions, ones which would be required to be pruned in the generation process.
After looking up a function and successfully matching **WARP** will also be able to apply type information.

##### Signature generation

SigKit's signature generation is provided through user python scripts located [here](https://github.com/Vector35/sigkit/tree/master).

Because of the separation of the signature generation and the core integration, the process becomes very cumbersome, specifically the process is too convoluted for smaller samples, and too slow for bigger samples.

#### What does this mean?

WARP can match on a greater number of functions which otherwise would be pruned at the generation process, this is not without its tradeoffs, we generate this function UUID on both ends, meaning that the algorithm must be carefully upgraded to ensure that previously generated UUID's are no longer valid.

Aside from just the matching of functions, we _never_ prune functions when added to the dataset this means we actually can store multiple functions for any given UUID. This is a major advantage for users who can now identify exactly what causes a collision and override, or otherwise understand more about the function.

After matching on a function successfully, we can reconstruct the function signature, not just the symbol name. SigKit has no information about the function calling convention or the function type.

[1]: https://devblogs.microsoft.com/oldnewthing/20110921-00/?p=9583
[2]: https://devblogs.microsoft.com/oldnewthing/20221109-00/?p=107373
[3]: https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-1-manual.pdf
[IDA FLIRT Documentation]: https://docs.hex-rays.com/user-guide/signatures/flirt/ida-f.l.i.r.t.-technology-in-depth
[Binary Ninja]: https://binary.ninja
[Binary Ninja Integration]: https://github.com/Vector35/binaryninja-api/tree/dev/plugins/warp