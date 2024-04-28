# Structs
## Ease Struct
A `Resource` which contains parameters for an easing function and has functions to manage them.
### Functions
#### progress_eased
Returns the return value of the easing function and parameters stored in the struct.
#### step
Increases the current step by one.
#### increase
Increases the current step by the amount provided.
#### new
Returns a new `EaseStruct` with the parameters provided.
#### is_done
Returns whether the easing function is done or not.
#### force_done
Set the easing function to be done.
# Enums
## EasingFunction
Defines which function to use for easing.
## EasingType
Defines whether to use a `In`, `Out` or `InOut` easing function.
