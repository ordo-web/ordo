module.exports = {
    mode: "production",

    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",

    entry: {
        'ordo': './src/index.ts',
        'ordo.min': './src/index.ts'
    },
    output: {
        filename: '[name].js',
        libraryTarget: 'umd',
        library: 'ordo',
        umdNamedDefine: true
    },

    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: [".ts", ".tsx", ".wasm", ".js"]
    },

    module: {
        rules: [
            /*{
                test: /\.ts(x?)$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader"
                    }
                ]
            },*/
            {
                test: /\.(ts|js)x?$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader'
                },
            },
            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            {
                enforce: "pre",
                test: /\.js$/,
                loader: "source-map-loader"
            },
        ]
    },

    experiments: {
        asyncWebAssembly: true,
        importAsync: true,
    },
};