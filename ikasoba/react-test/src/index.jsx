import { render } from "preact/";
import { useState } from "preact/hooks";

function App() {
  const [count, setCount] = useState(0);

  return (
    <button onClick={() => setCount(count + 1)}>
      count: {count}
    </button>
  );
}

window.addEventListener("load", () => {
  render(<App />, document.body);
});