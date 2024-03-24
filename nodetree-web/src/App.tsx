import NTEditor from "./components/NTEditor";
import NTTree from "./components/NTTree";

function App() {
  return (
    <div className="flex flex-row p-5">
      <div className="w-1/2">
        <NTTree />
      </div>

      <div className="w-1/2 h-full">
        <NTEditor />
      </div>
    </div >
  );
}

export default App;
