// Define an interface for the window object with the 'env' property
interface WindowWithEnv {
    env: {
        production: boolean
        backendUrl?: string;
        debug?: boolean;
    };
}

// Extend the global Window interface with the custom window object
declare global {
    interface Window extends WindowWithEnv { }
}

export const environment = {
    production: window.env.production ?? false,
    backendUrl: window.env.backendUrl ?? "http://localhost:3000",
    debug: window.env.debug ?? false,
};