import { Routes, Route, Navigate } from "react-router-dom";
import { useAuth } from "./contexts/AuthContext";
import Login from "./components/Login";
import StudioRoom from "./pages/StudioRoom";
import FloatingStudio from "./pages/FloatingStudio";

function App() {
  const { user, loading } = useAuth();

  if (loading) {
    return (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          height: "100vh",
          color: "#555",
          fontSize: "14px",
        }}
      >
        Loading...
      </div>
    );
  }

  return (
    <Routes>
      <Route
        path="/"
        element={user ? <Navigate to="/studio" /> : <Login />}
      />
      <Route
        path="/studio"
        element={user ? <StudioRoom /> : <Navigate to="/" />}
      />
      {/* /floating is opened by the Rust backend as a separate WebviewWindow */}
      <Route
        path="/floating"
        element={user ? <FloatingStudio /> : <Navigate to="/" />}
      />
      <Route path="*" element={<Navigate to="/" />} />
    </Routes>
  );
}

export default App;
