import { create } from "zustand"

interface WasmStoreState {
  wasm: Wasm | null
  initWasm: () => void
}

interface MyStoreState {
  bears: number
  increasePopulation: () => void
  removeAllBears: () => void
}

// IMPORTANT: when updating path, don't forget to update both references in this file,
// apparently can't avoid the repetition?
export type Wasm = typeof import("../wasm/mol")

export const useMyStore = create<MyStoreState>((set) => ({
  bears: 0,
  increasePopulation: () => set((state) => ({ bears: state.bears + 1 })),
  removeAllBears: () => set({ bears: 0 }),
}))

export const useWasmStore = create<WasmStoreState>((set) => ({
  wasm: null,
  initWasm: async () => {
    const wasm: Wasm = await import("../wasm/mol")
    set({ wasm })
  },
}))
