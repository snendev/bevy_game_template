# Blueprints

This is the base layer, which defines any data types needed by everything else.

The components defined in this crate typically are used as query filters or initializers
for more complicated bundles. In this way each crate can handle the existence of these
"blueprint" components reactively.

N.B./todo I would rename this directory "blueprints" but for some reason it freaks out my rust-analyzer.
