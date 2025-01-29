import { createContext, useState, useMemo } from "react";
import { createTheme } from "@mui/material/styles"

// color design tokens
export const tokens = (mode) => ({
    ...(mode === 'dark'
        ? {
            primary: {
                100: "#d0d0d1", 
                200: "#a1a2a2",
                300: "#737374",
                400: "#444545",
                500: "#151617",
                600: "#111212",
                700: "#0d0d0e",
                800: "#080909",
                900: "#040405"
            },
            gray: {
                100: "#d2d2d2",
                200: "#a5a5a6",
                300: "#777979",
                400: "#4a4c4d",
                500: "#1d1f20",
                600: "#17191a",
                700: "#111313",
                800: "#0c0c0d",
                900: "#060606"
            },
            blueAccent: {
                100: "#ccecff",
                200: "#99daff",
                300: "#66c7ff",
                400: "#33b5ff",
                500: "#00a2ff",
                600: "#0082cc",
                700: "#006199",
                800: "#004166",
                900: "#002033"
            },
            orangeAccent: {
                100: "#ffebcc",
                200: "#ffd799",
                300: "#ffc266",
                400: "#ffae33",
                500: "#ff9a00",
                600: "#cc7b00",
                700: "#995c00",
                800: "#663e00",
                900: "#331f00"
            },
            redAccent: {
                100: "#f6d0d0",
                200: "#eda1a0",
                300: "#e47171",
                400: "#db4241",
                500: "#d21312",
                600: "#a80f0e",
                700: "#7e0b0b",
                800: "#540807",
                900: "#2a0404"
            },
        }
        : {
            primary: {
                100: "#040405",
                200: "#080909",
                300: "#0d0d0e",
                400: "#111212",
                500: "#151617",
                600: "#444545",
                700: "#737374",
                800: "#a1a2a2",
                900: "#d0d0d1"
            },
            gray: {
                100: "#272727",
                200: "#4e4e4e",
                300: "#747474",
                400: "#9b9b9b",
                500: "#c2c2c2",
                600: "#cecece",
                700: "#dadada",
                800: "#e7e7e7",
                900: "#f3f3f3"
            },
            blueAccent: {
                100: "#002033",
                200: "#004166",
                300: "#006199",
                400: "#0082cc",
                500: "#00a2ff",
                600: "#33b5ff",
                700: "#66c7ff",
                800: "#99daff",
                900: "#ccecff"
            },
            orangeAccent: {
                100: "#331f00",
                200: "#663e00",
                300: "#995c00",
                400: "#cc7b00",
                500: "#ff9a00",
                600: "#ffae33",
                700: "#ffc266",
                800: "#ffd799",
                900: "#ffebcc"
            },
            redAccent: {
                100: "#2a0404",
                200: "#540807",
                300: "#7e0b0b",
                400: "#a80f0e",
                500: "#d21312",
                600: "#db4241",
                700: "#e47171",
                800: "#eda1a0",
                900: "#f6d0d0"
            },
        }),
})


// mui theme settings
export const themeSettings = (mode) => {
    const colors = tokens(mode);

    return {
        palette: {
            mode: mode,
            ...(mode === 'dark'
                ? {
                    primary: {
                        main: colors.primary[500],
                    },
                    secondary: {
                        main: colors.blueAccent[500],
                    },
                    neutral: {
                        dark: colors.gray[700],
                        main: colors.gray[500],
                        light: colors.gray[100],
                    },
                    background: {
                        default: colors.gray[900]
                    }
                } : {
                    primary: {
                        main: colors.primary[500],
                    },
                    secondary: {
                        main: colors.blueAccent[500],
                    },
                    neutral: {
                        dark: colors.gray[700],
                        main: colors.gray[500],
                        light: colors.gray[100],
                    },
                    background: {
                        default: colors.gray[500]
                    }
                }),
        },
        typography: {
            fontFamily: ["Barlow Semi Condensed", "sans-serif"].join(","),
            fontSize: 12,
            h1: {
                fontFamily: ["Barlow Semi Condensed", "sans-serif"].join(","),
                fontSize: 40,
            },
            h2: {
                fontFamily: ["Barlow Semi Condensed", "sans-serif"].join(","),
                fontSize: 32,
            },
            h3: {
                fontFamily: ["Barlow Semi Condensed", "sans-serif"].join(","),
                fontSize: 24,
            },
            h4: {
                fontFamily: ["Barlow Semi Condensed", "sans-serif"].join(","),
                fontSize: 20,
            },
            h5: {
                fontFamily: ["Barlow Semi Condensed", "sans-serif"].join(","),
                fontSize: 16,
            },
            h6: {
                fontFamily: ["Barlow Semi Condensed", "sans-serif"].join(","),
                fontSize: 14,
            },
        },
    };
};

export const ranks = [
    "#5A50C87D",
    "#3B60C97D",
    "#3975BB7D",
    "#6A91987D",
    "#9AAC757D",
    "#C4C4577D",
    "#F2DA367D",
    "#EDB73D7D",
    "#E894447D",
    "#E5744A7D",
    "#E051517D",
]

//context for color mode
export const ColorModeContext = createContext({
    toggleColorMode: () => {}
});

export const useMode = () => {
    const[mode, setMode] = useState("dark");

    const colorMode = useMemo(
        () => ({
            toggleColorMode: () =>
            setMode((prev) => (prev === "light" ? "dark" : "light")),
        }),
        []
    );

    const theme = useMemo(() => createTheme(themeSettings(mode)), [mode])

    return [theme, colorMode];
}