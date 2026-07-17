// swift-tools-version: 6.1
import PackageDescription

let package = Package(
    name: "PronoteKit",
    platforms: [.iOS(.v16)],
    products: [
        .library(name: "PronoteKit", targets: ["PronoteKit"])
    ],
    targets: [
        .binaryTarget(
            name: "pronoteFFI",
            path: "./pronote.xcframework"
        ),
        .target(
            name: "PronoteKit",
            dependencies: [.target(name: "pronoteFFI")],
            path: "Sources/PronoteKit"
        ),
    ]
)
