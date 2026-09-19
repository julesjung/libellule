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
    private let appParameters: AppParameters?
    
    init() {
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
        .modelContainer(Store.container)
    }
}
