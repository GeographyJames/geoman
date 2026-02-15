import { Select } from "@/components/forms/components/Select";
import { ShapefilePreview } from "@/components/forms/components/ShapefilePreview";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useCollections } from "@/hooks/api/useCollections";
import { useForm } from "react-hook-form";
import { useEffect, useMemo, useState } from "react";
import { Shapefile } from "@/lib/shapefile";
import { usePostProjectFeature } from "@/hooks/api/usePostProjectFeature";
import { useAddFeature } from "../../contexts/AddFeatureContext";
import { ApiError } from "@/lib/api";
import type { FeatureCollection } from "geojson";
import { parseShp, parseDbf, combine } from "shpjs";

function parseEpsgCode(prj: string): number | null {
  const match = prj.match(/AUTHORITY\["EPSG","(\d+)"\]/);
  return match ? Number(match[1]) : null;
}

const COMPATIBLE_GEOMETRY: Record<string, string[]> = {
  Point: ["Point"],
  MultiPoint: ["Point", "MultiPoint"],
  LineString: ["LineString"],
  MultiLineString: ["LineString", "MultiLineString"],
  Polygon: ["Polygon"],
  MultiPolygon: ["Polygon", "MultiPolygon"],
};

const SINGLE_GEOMETRY_TYPES = ["Point", "LineString", "Polygon"];

const AddSiteFeatureInner = () => {
  const { project, clear } = useAddFeature();
  const { data: collections } = useCollections();
  const { addError, clearErrors, closeDialog } = useModal();
  const { mutate: postFeature, isPending } = usePostProjectFeature();
  const { register, watch, setValue, reset } = useForm();
  const files = watch("files") as FileList;
  const name = watch("name") as string;
  const [geojson, setGeojson] = useState<FeatureCollection | null>(null);
  const [prjText, setPrjText] = useState<string | null>(null);
  const [nullGeometryCount, setNullGeometryCount] = useState(0);
  const [selectedCollectionId, setSelectedCollectionId] = useState<
    string | null
  >(null);

  useEffect(() => {
    if (collections?.length && !selectedCollectionId) {
      setSelectedCollectionId(String(collections[0].id));
    }
  }, [collections, selectedCollectionId]);

  const selectedCollection = collections?.find(
    (c) => String(c.id) === selectedCollectionId,
  );
  const shapefileGeometryType = geojson?.features[0]?.geometry.type ?? null;

  const emptyShapefile = geojson !== null && geojson.features.length === 0;

  const projectSrid = project?.outputDto.properties.crs_srid ?? null;
  const shapefileSrid = prjText ? parseEpsgCode(prjText) : null;
  console.log(`project crs = ${projectSrid} shapefile crs = ${shapefileSrid}`);
  const willReproject =
    projectSrid !== null &&
    shapefileSrid !== null &&
    projectSrid !== shapefileSrid;

  const tooManyFeatures =
    selectedCollection != null &&
    geojson != null &&
    SINGLE_GEOMETRY_TYPES.includes(selectedCollection.geometry_type) &&
    geojson.features.length > 1;

  const geometryMismatch = useMemo(() => {
    if (!selectedCollection || !shapefileGeometryType) return null;
    const allowed = COMPATIBLE_GEOMETRY[selectedCollection.geometry_type];
    if (allowed && !allowed.includes(shapefileGeometryType)) {
      return `Shapefile geometry (${shapefileGeometryType}) is not compatible with collection type (${selectedCollection.geometry_type})`;
    }
    return null;
  }, [selectedCollection, shapefileGeometryType]);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!project) return;
    const formData = new FormData(e.currentTarget);
    const collectionId = Number(formData.get("collection"));
    const name = formData.get("name") as string;
    const shapefile = Shapefile.fromFieldValues({ files });
    if (typeof shapefile === "string") {
      addError(shapefile);
      return;
    }
    postFeature(
      { projectId: project.id, collectionId, name, shapefile },
      {
        onSuccess: () => {
          reset();
          setGeojson(null);
          setNullGeometryCount(0);
          setPrjText(null);
          closeDialog();
          clear();
        },
        onError: (error) => {
          if (error instanceof ApiError && error.long_message) {
            addError(error.long_message);
          } else {
            addError(`Unable to add feature: ${error.message}`);
          }
        },
      },
    );
  };

  useEffect(() => {
    if (files instanceof FileList && files.length > 0) {
      clearErrors();
      const result = Shapefile.fromFilesList(files);
      if (typeof result === "string") {
        addError(result);
        setValue("name", null);
        setGeojson(null);
        setNullGeometryCount(0);
        setPrjText(null);
        return;
      }
      setValue("name", result.filename);

      Promise.all([
        result.shp.arrayBuffer(),
        result.dbf.arrayBuffer(),
        result.prj.text(),
      ])
        .then(([shp, dbf, prj]) => {
          return [combine([parseShp(shp, prj), parseDbf(dbf)]), prj] as const;
        })
        .then(([fc, prj]) => {
          const withGeometry = fc.features.filter((f) => f.geometry != null);
          setNullGeometryCount(fc.features.length - withGeometry.length);
          setGeojson({ ...fc, features: withGeometry } as FeatureCollection);
          setPrjText(prj);
        })
        .catch(() => {
          setGeojson(null);
          setNullGeometryCount(0);
          setPrjText(null);
          addError("Failed to parse shapefile for preview");
        });
    }
  }, [files]);

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <Select
        name="collection"
        label="Select collection"
        required={true}
        onChange={setSelectedCollectionId}
        value={selectedCollectionId ?? undefined}
      >
        {collections?.map((c) => (
          <option key={c.id} value={c.id}>
            {`${c.title} (${c.geometry_type})`}
          </option>
        ))}
      </Select>
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Shapefiles</legend>
        <input
          {...register("files")}
          multiple={true}
          className="file-input file-input-bordered w-full"
          type="file"
          required
        />
      </fieldset>
      <ShapefilePreview
        geojson={geojson}
        prj={prjText}
        nullGeometryCount={nullGeometryCount}
      />
      {emptyShapefile && (
        <div role="alert" className="alert alert-warning text-sm">
          Shapefile contains no features
        </div>
      )}
      {geometryMismatch && (
        <div role="alert" className="alert alert-warning text-sm">
          {geometryMismatch}
        </div>
      )}
      {tooManyFeatures && (
        <div role="alert" className="alert alert-warning text-sm">
          Shapefile has {geojson?.features.length} features but {selectedCollection?.geometry_type} collections only accept a single feature
        </div>
      )}
      {willReproject && (
        <div role="alert" className="alert alert-info text-sm">
          Features will be reprojected from EPSG:{shapefileSrid} to project CRS
          EPSG:{projectSrid}
        </div>
      )}
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Name</legend>
        <div className="flex items-center gap-2">
          <input
            className="input input-bordered w-full"
            {...register("name")}
            required
            type="text"
          />
        </div>
      </fieldset>
      <div className="modal-action">
        <CancelButton
          onClick={() => {
            reset();
            setGeojson(null);
            setNullGeometryCount(0);
            setPrjText(null);
            closeDialog();
            clear();
          }}
          disabled={isPending}
        />
        <SubmitButton
          text="Add feature"
          loadingText="Adding..."
          loading={isPending}
          disabled={
            !geojson ||
            !name?.trim() ||
            !!geometryMismatch ||
            emptyShapefile ||
            tooManyFeatures
          }
        />
      </div>
    </form>
  );
};

export const AddSiteFeatureForm = () => {
  return (
    <Modal id="add_site_feature" title="Add site feature">
      <AddSiteFeatureInner />
    </Modal>
  );
};
