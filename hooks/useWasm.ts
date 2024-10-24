import { useEffect } from "react"
import { useWasmStore } from "./store"

export const useWasm = () => {
  const { wasm, initWasm } = useWasmStore()

  useEffect(() => {
    if (!wasm) {
      initWasm()
    }
  }, [wasm, initWasm])

  return wasm
}
