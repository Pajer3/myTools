import { invoke } from "@tauri-apps/api/core";
import "../styles/cards/card.css";

interface Workflow {
    title: string;
    icon: string;
}

const workflows: Workflow[] = [
    { title: "School Enviroment", icon: "🎓" },
    { title: "Work Enviroment", icon: "💼" },
    { title: "Hack Enviroment", icon: "💻" },
];

export default function WorkflowCards() {

    async function handleWorkflow(title: string) {
        console.log(`Triggering workflow: ${title}`);
        await invoke("execute_workflow", { name: title });
    }

    async function handleFileManager() {
        console.log("Opening file manager");
        await invoke("open_file_manager");
    }

    return (
        <div className="workflow-wrapper">
            <div className="card-container">
                {workflows.map((wf, index) => (
                    <div key={index} className="card" onClick={() => handleWorkflow(wf.title)}>
                        <div className="card-icon">{wf.icon}</div>
                        <h3>{wf.title}</h3>
                    </div>
                ))}
            </div>

            <div className="action-buttons">
                <button className="glass-btn" onClick={handleFileManager}>
                    <span>📂</span> Open Filemanager
                </button>
            </div>
        </div>
    );
}
