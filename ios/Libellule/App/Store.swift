//
//  Store.swift
//  Libellule
//
//  Created by Jules on 19/09/2026.
//

import Foundation
import SwiftData

nonisolated enum Store {
    static let container = makeContainer()
    
    private static func makeContainer() -> ModelContainer {
        let schema = Schema([CachedDay.self, CachedHomework.self, CachedMenu.self])
        let configuration = ModelConfiguration()
        
        if let container = try? ModelContainer(for: schema, configurations: configuration) {
            return container
        }
        
        try? FileManager.default.removeItem(at: configuration.url)        
        if let container = try? ModelContainer(for: schema, configurations: configuration) {
            return container
        }
        
        return try! ModelContainer(for: schema, configurations: ModelConfiguration(isStoredInMemoryOnly: true))
    }
}
