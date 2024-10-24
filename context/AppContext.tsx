"use client"

import React, { createContext, ReactNode } from "react"

const initial: IAppContext = {}

export const AppContext = createContext(initial)

export const AppContextProvider: React.FC<AppContextProviderProps> = ({
  children,
}) => {
  const deps = { foo: 123 }
  return (
    <AppContext.Provider value={{ deps: deps }}>{children}</AppContext.Provider>
  )
}

interface IAppContext {
  // optional just for ts, which needs a default state
  // this is set synchronously so we expect it to be always set
  // TODO better solution?
  deps?: Deps
}

export interface Deps {
  foo: number
}

export type Wasm = typeof import("../wasm/mol")

interface AppContextProviderProps {
  children: ReactNode
}
