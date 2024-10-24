const nextConfig = {
  webpack: (config) => {
    config.experiments = {
      ...config.experiments,
      syncWebAssembly: true,
    };
    return config;
  },
};

export default nextConfig;
