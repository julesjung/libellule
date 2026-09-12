//
//  LibelluleApp.swift
//  Libellule
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI
import SwiftData

@main
struct LibelluleApp: App {
    private let modelContainer: ModelContainer
    private let syncService: SyncService
    private let appParameters: AppParameters?
    
    init() {
        let schema = Schema([CachedDay.self])
        let configuration = ModelConfiguration(schema: schema)
        
        do {
            modelContainer = try ModelContainer(for: schema, configurations: [configuration])
        } catch {
            try? FileManager.default.removeItem(at: configuration.url)
            modelContainer = try! ModelContainer(for: schema, configurations: [configuration])
        }
        
        syncService = SyncService(modelContainer: modelContainer)
        
        if let data = UserDefaults.standard.data(forKey: "app") {
            appParameters = try? JSONDecoder().decode(AppParameters.self, from: data)
        } else {
            appParameters = nil
        }
    }
    
    var body: some Scene {
        WindowGroup {
            RootView(appParameters: appParameters)
        }
        .environment(\.syncService, syncService)
        .modelContainer(modelContainer)
    }
}
