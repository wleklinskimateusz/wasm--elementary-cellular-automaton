# wasm-elementary-cellular-automaton

A simple elementary cellular automaton implemented in Rust and compiled to WebAssembly.

## Adding to JS/TS project

- Add to `package.json`:

```bash
npm install wasm-elementary-cellular-automaton
```

You can use bun, pnpm, yarn, deno or any other package manager.

- Import in your JS/TS code:

```typescript
import { CellularAutomaton } from "wasm-elementary-cellular-automaton";
```

- Use it:

Example with React and TailwindCSS:
```typescript
const App = () => {
    const initialState = new Uint8Array(128).fill(0);
    initialState[0] = 1;
    
    const automaton = new CellularAutomaton(128, initialState, false);
    const [vector, setVector] = useState(automaton.get_state());
    useEffect(() => {
        const interval = setInterval(() => {
            automaton.step();
            setVector(automaton.get_state());
        }, 100);
        return () => clearInterval(interval);
    }, []);
    return (
        <div>
            {vector.map((cell, index) => <div key={index} className={cell ? 'bg-black' : 'bg-white' + " w-4 h-4"} />)}
        </div>
    )
}
```

Full example you can find here: https://github.com/wleklinskimateusz/cellular-automaton-react

Source code for the binding is here: https://github.com/wleklinskimateusz/wasm--elementary-cellular-automaton

## License

MIT License


