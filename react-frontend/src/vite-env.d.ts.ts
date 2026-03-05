declare const __URLS__: {
    health_check: string
    health_check_authenticated: string
    api: {
        base: string
        keys: string
        projects: string
        users: string
        collections: string
        project_features: string
        app_settings: string
        teams: string
        business_units: string
        epsg: string
        turbine_layouts: string
        data_providers: string
        data_provider_services: string
        data_provider_layers: string
    }
    docs: { base: string }
    ogc_api: {
        base: string
        collections: string
        project: string
        conformance_declaration: string
        openapi: string
    }
    webhooks: { base: string, clerk: string }
}

declare const __RUN_ENVIRONMENT__: string;
