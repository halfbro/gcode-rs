initSidebarItems({"enum":[["ConstantOp","Operations performed on the constant portion of the [conversion factor][factor]. Used to help guide optimizations when floating point underlying storage types are used."]],"macro":[["ISQ","Macro to implement `quantity` type aliases for a specific [system of units][units] and value storage type."],["impl_from","`impl_from` generates generic inter-Kind implementations of `From`."],["prefix","Macro to implement the [SI][si] prefixes for [multiples of units][mult] and [submultiples of units][submult]."],["quantity","Macro to implement a [quantity][quantity] and associated [measurement units][measurement]. Note that this macro must be executed in direct submodules of the module where the `system!` macro was executed. `@...` match arms are considered private."],["storage_types","Macro to duplicate code a per-storage type basis. The given code is duplicated in new modules named for each storage type. A type alias, `V`, is generated that code can use for the type. `@...` match arms are considered private."],["system","Macro to implement a system of quantities. `@...` match arms are considered private."]],"mod":[["fmt","Utilities for formatting and printing quantities."],["marker","Primitive traits and types representing basic properties of types."],["si","[International System of Units][si] (SI) and [International System of Quantities][isq] (ISQ) implementations."],["str","Unicode string slice manipulation for quantities."]],"trait":[["Conversion","Trait to identify [units][units] which have a [conversion factor][factor]."],["ConversionFactor","Trait representing a [conversion factor][factor]."],["Kind","Default [kind][kind] of quantities to allow addition, subtraction, multiplication, division, remainder, negation, and saturating addition/subtraction."]]});