# Controls

This is the controller layer which integrates leafwing-input-manager and manages devices.

This crates values can generate valid character actions for gameplay processing. It is notably
separate from other types of controllers, such as AI controllers or network controllers. Attach a
`Controller` to a player entity and these systems will translate key controls into valid character
action components. 
