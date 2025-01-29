import { useCallback, useState } from "react";

export default function DropdownHandler(initialState) { 
    const [isOpen, setOpenState] = useState(initialState)

    const toggle = useCallback (() => {
        setOpenState((state) => !state);
    }, [setOpenState]);

    return {isOpen, toggle}
}