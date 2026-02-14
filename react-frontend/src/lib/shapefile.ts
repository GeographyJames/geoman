import type { FieldValues } from "react-hook-form"

export class Shapefile {
    filename: string
    shp: File
    dbf:File
    prj: File
    shx: File
    constructor(filename: string, shp:File, dbf:File, prj:File, shx:File) {
        this.filename=filename
        this.shp=shp,
        this.dbf=dbf,
        this.prj=prj,
        this.shx=shx
    }

    static fromFilesList(files: FileList): Shapefile | string {
        if (files.length === 0) {
             return "no files provided"
        }
        let shapefileName = files[0].name.split(".")[0]
        const filesMap: Map<string, File> = new Map();
        
        for (const file of files) {
            let filenameArray = file.name.split(".")
            if (filenameArray.length < 2)
            return (`Invalid filename (${file.name}). Filename must have a name and a file extension.`)
            if (filenameArray[0] !==shapefileName) {
                return("Filenames do not match")
            }
            filesMap.set(filenameArray[filenameArray.length -1], file)
        }
        for (const suffix of ["shp", "dbf", "prj", "shx"]) {

            if (!filesMap.has(suffix)) {
                return(`Missing .${suffix} component`)
            }
        }
        
    return new Shapefile(
        shapefileName,
        filesMap.get("shp")!,
        filesMap.get("dbf")!,
        filesMap.get("prj")!,
        filesMap.get("shx")!
      );
    }
     
    addToForm(form: FormData) {
        form.append("shp", this.shp)
        form.append("dbf", this.dbf)
        form.append("prj", this.prj)
        form.append("shx", this.shx)

    }

    static fromFieldValues(data: FieldValues): Shapefile | string {
        const files = data.files as FileList
        if (!files) {return "no files"}
        return Shapefile.fromFilesList(files)
        }
    
}