import React from "react";
import {type Dispatch } from "react";

export interface ShowArchivedType {
    showArchived: boolean,
    dispatch: Dispatch<boolean>
}



export const ShowArchivedProjectsContext = React.createContext<ShowArchivedType>({} as ShowArchivedType)
export  const ShowArchivedBoundariesContext = React.createContext<ShowArchivedType>({} as ShowArchivedType)
export const ShowArchivedLayoutsContext = React.createContext<ShowArchivedType>({} as ShowArchivedType)
export const ShowArchivedSearchAreasContext = React.createContext<ShowArchivedType>({} as ShowArchivedType)
export const ShowArchivedFiguresContext = React.createContext<ShowArchivedType>({} as ShowArchivedType)