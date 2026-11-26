; A CANopen electronic data sheet's named blocks are its sections: file and device metadata
; (`[DeviceInfo]`) and one per object-dictionary entry (`[1000]`, `[1018sub1]`). Keys inside a
; section are fixed by CiA 306, so they are data rather than declarations.
(section (section_name) @name) @definition.module
