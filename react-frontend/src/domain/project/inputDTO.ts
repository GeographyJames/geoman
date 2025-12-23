export default interface ProjectInputDTO{
    name: string,
    visibility?: string,
    crs_srid?: number,
    country_code: string
    slug: string
    technologies?: number[]

}