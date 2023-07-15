// App.tsx
import { createSignal } from "solid-js";
import { clear, addDigit, setOperation, calculate } from './calculate';
import { Property } from "csstype";

const App = () => {
  const [display, setDisplay] = createSignal("0");

  const handleDigit = async (digit: number) => {
    await addDigit(digit);
    setDisplay(digit.toString());
  };

  const handleOperation = async (operation: string) => {
    await setOperation(operation);
    setDisplay(operation);
  };

  const handleClear = async () => {
    await clear();
    setDisplay("0");
  };

  const handleCalculate = async () => {
    const result = await calculate();
    setDisplay(result.toString());
  };

  const buttonStyle = {
    height: "20px",
    padding: "1rem"
  }

  const buttonContainerStyle = {
    display: "flex",
    "flex-direction": "column" as Property.FlexDirection,
    "justify-content": "center",
    "align-items": "center",
    width: "100%",
    height: "100%"
  }

  return (
    <div style={{
      display: "flex",
      "flex-direction": "row",
      "justify-content": "center",
      "align-items": "center",
      height: "100vh"
    }}>
      <div style={{
        display: "flex",
        "flex-direction": "column",
        padding: "1rem",
        width: "calc(50% - 2rem)",
        border: "1px solid black",
        "border-radius": "1rem",
        gap: "0.5rem"
      }}>
        <h1 style={{
          "text-align": "center"
        }}>Simple Calculator</h1>
        <div style={{
          height: "20px",
          "background-color": "grey",
          color: "white",
          padding: "1rem",
        }}>{display()}</div>
        <div style={{
          display: "grid",
          "grid-template-columns": "auto auto auto auto"
        }}>
          {[1, 2, 3, 4, 5, 6, 7, 8, 9, 0].map((digit) => (
            <button style={buttonStyle} onClick={() => handleDigit(digit)}>
              <div style={buttonContainerStyle}>
                {digit}
              </div>
            </button>
          ))}
          {['+', '-', '*', '/'].map((operation) => (
            <button style={buttonStyle} onClick={() => handleOperation(operation)}>
              <div style={buttonContainerStyle}>
                {operation}
              </div>
            </button>
          ))}
          <button style={buttonStyle} onClick={handleCalculate}>
            <div style={buttonContainerStyle}>
              =
            </div>
          </button>
          <button style={buttonStyle} onClick={handleClear}>
            <div style={buttonContainerStyle}>
              C
            </div>
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
