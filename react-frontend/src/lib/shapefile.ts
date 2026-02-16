import type { FieldValues } from "react-hook-form"

export class Shapefile {
    filename: string
    shp?: File
    dbf?: File
    prj?: File
    shx?: File
    shz?: File

    private constructor(filename: string, files: { shp?: File; dbf?: File; prj?: File; shx?: File; shz?: File }) {
        this.filename = filename
        this.shp = files.shp
        this.dbf = files.dbf
        this.prj = files.prj
        this.shx = files.shx
        this.shz = files.shz
    }

    get isZipped(): boolean {
        return this.shz != null
    }

    static fromFilesList(files: FileList): Shapefile | string {
        if (files.length === 0) {
             return "no files provided"
        }

        if (files.length === 1) {
            const file = files[0]
            const ext = file.name.split(".").pop()?.toLowerCase()
            if (ext === "shz") {
                const name = file.name.split(".").slice(0, -1).join(".")
                return new Shapefile(name, { shz: file })
            }
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
        {
            shp: filesMap.get("shp")!,
            dbf: filesMap.get("dbf")!,
            prj: filesMap.get("prj")!,
            shx: filesMap.get("shx")!,
        }
      );
    }

    addToForm(form: FormData) {
        if (this.shz) {
            form.append("shz", this.shz)
        } else {
            form.append("shp", this.shp!)
            form.append("dbf", this.dbf!)
            form.append("prj", this.prj!)
            form.append("shx", this.shx!)
        }
    }

    static fromFieldValues(data: FieldValues): Shapefile | string {
        const files = data.files as FileList
        if (!files) {return "no files"}
        return Shapefile.fromFilesList(files)
        }

}