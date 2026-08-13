//
//  LibelluleApp.swift
//  Libellule
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI

@main
struct LibelluleApp: App {
    @State private var session = SessionStore()
    
    var body: some Scene {
        WindowGroup {
            RootView()
                .environment(session)
                .task {
                    guard let url = UserDefaults.standard.string(forKey: "url") else {
                        session.state = .loggedOut
                        return
                    }
                    
                    await session.connect(url: url)
                }
        }
    }
}
