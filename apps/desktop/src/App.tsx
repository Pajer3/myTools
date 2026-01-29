import Header from "./components/header";
import WorkflowCards from "./workflows/WorkflowCards";

export default function App() {
  return (
    <div style={{ width: "100%", height: "100%", padding: 0 }}>
      <Header />
      <WorkflowCards />
    </div>
  );
}
