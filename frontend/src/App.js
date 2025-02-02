import { ColorModeContext, useMode } from "./theme";
import { CssBaseline, ThemeProvider} from "@mui/material";
import ScoreUpdates from "./scenes/ScoreUpdates";
import Overall from "./scenes/Overall";
import SPMap from "./scenes/SPMap"
import {
  BrowserRouter as Router,
  Routes,
  Route,
} from "react-router-dom";

function App() {
  const [theme, colorMode] = useMode();
  
  return (
    <Router>
      <ColorModeContext.Provider value={colorMode}>
        <ThemeProvider theme={theme}>
          <CssBaseline />
          <div className="app">
            <main className="content">
              <Routes>
                <Route exact path="/" element={<ScoreUpdates />} />
                <Route exact path="/leaderboard/overall" element={<Overall />} />
                <Route path="/map/sp/:levelId" element={<SPMap />} />
              </Routes>
            </main>
          </div>
        </ThemeProvider>
      </ColorModeContext.Provider>
    </Router>
  );
}

export default App;