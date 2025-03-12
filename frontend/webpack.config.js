const path = require('path');

module.exports = {
    entry: {
        addClass: './class/addClass.js',
        addRoom: './room/addRoom.js',
        addFaculty: './faculty/addFaculty.js',
        addSchedule: './schedule/addSchedule.js',
        addFeature: './feature/addFeature.js',
        
    },

    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js',
    },

    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['@babel/preset-env'],
                    },
                },
            },
            {
                test: /\.css$/,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },

    devServer: {
        contentBase: path.join(__dirname, 'dist'),
        compress: true,
        port: 9000,
        proxy: {
            '/api': 'http://localhost:5000',
        },
    },
};