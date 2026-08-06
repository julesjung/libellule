//
//  ContentView.swift
//  Pronote
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI
import PronoteKit

struct ContentView: View {
    @State private var state: AppState = {
        if let url = UserDefaults.standard.string(forKey: "url") {
            return .connecting(url: url)
        }
        
        return .loggedOut
    }()
    
    var body: some View {
        switch state {
        case .loggedOut:
            InstanceSelector(state: $state)
        case .connecting(let url):
            ProgressView("Connexion à l'instance PRONOTE")
                .task {
                    let instance = try! await Instance(url: url)
                    UserDefaults.standard.set(url, forKey: "url")
                    self.state = .connected(instance: instance)
                }
        case .connected(let instance):
            LoginView(instance: instance, state: $state)
        case .authenticating(let instance, let username, let password):
            ProgressView("Authentification")
                .task {
                    let client = try! await Client(instance: instance, username: username, password: password)
                    self.state = .authenticated(client: client)
                }
        case .authenticated(let client):
            HomeView(client: client)
        }
    }
}

enum AppState {
    case loggedOut
    case connecting(url: String)
    case connected(instance: Instance)
    case authenticating(instance: Instance, username: String, password: String)
    case authenticated(client: Client)
}
