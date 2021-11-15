import {
  BrowserRouter as Router,
  NavLink,
  Route,
  Routes,
} from "react-router-dom";
import { Containers } from "./Containers";
import { Overview } from "./Overview";

export const App = () => {
  return (
    <Router>
      <nav className="nav">
        <div className="container">
          <div className="nav-list">
            <NavLink className="nav-item" to="/">
              Overview
            </NavLink>
            <NavLink className="nav-item" to="/containers">
              Containers
            </NavLink>
          </div>
        </div>
      </nav>
      <main>
        <div className="container">
          <Routes>
            <Route path="/" element={<Overview />} />
            <Route path="/containers" element={<Containers />} />
          </Routes>
        </div>
      </main>
    </Router>
  );
};
