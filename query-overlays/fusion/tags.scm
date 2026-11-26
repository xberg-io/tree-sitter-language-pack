; Prototypes a Fusion file declares, plus the prototypes it instantiates or inherits from.

; `prototype(Vendor.Site:Card) < prototype(Neos.Fusion:Component) { ... }`
(property (path (prototype_signature (type) @name))) @definition.class

; `prototype(Vendor.Site:Card) < prototype(Neos.Fusion:Component)`
(copy source: (prototype_signature (type) @name)) @reference.class

; `root = Neos.Fusion:Case { ... }`
(prototype_instance (type) @name) @reference.class
