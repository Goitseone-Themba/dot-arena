import { Route, Routes } from "react-router-dom"
import { GamePlay } from "./pages/GamePlay"
import { Home } from "./pages/Home"
import { MatchConfirm } from "./pages/MatchConfirm"
import { Queue } from "./pages/Queue"
import { Register } from "./pages/Register"
import { Result } from "./pages/Results"

function App() {

    return (
            <Routes>
                <Route index element={<Home />} />
                <Route path="/register" element={<Register />} />
                <Route path="/queue" element={<Queue />} />
                <Route path="/confirm" element={<MatchConfirm />} />
                <Route path="/play" element={<GamePlay />} />
                <Route path="/results" element={<Result />} />
            </Routes>
    )
}

export default App
