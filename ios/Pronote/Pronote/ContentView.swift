//
//  ContentView.swift
//  Pronote
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI
import PronoteKit

struct ContentView: View {
    // TODO: store user state in persistent storage
    @State private var state: AppState = .instanceSelection
    @State private var instance: Instance?
    @State private var client: Client?
    
    var body: some View {
        switch state {
        case .instanceSelection:
            InstanceSelector(instance: $instance, state: $state)
        case .authentication:
            LoginView(instance: instance!, client: $client, state: $state)
        case .home:
            MainView(client: client!)
        }
    }
}

enum AppState: String {
    case instanceSelection
    case authentication
    case home
}

#Preview {
    ContentView()
}
