//
//  RootView.swift
//  Libellule
//
//  Created by Jules on 10/08/2026.
//

import SwiftUI

struct RootView: View {
    @Environment(SessionStore.self) private var session
    
    var body: some View {
        switch session.state {
        case .loggedOut:
            InstanceView()
        case .connecting, .authenticating:
            ProgressView()
        case .connected(let instance):
            LoginView(instance: instance)
        case .authenticated(let client):
            HomeView(client: client)
        case .failed(let error):
            ContentUnavailableView {
                Label("Erreur", systemImage: "wifi.exclamationmark")
            } description: {
                Text(error.localizedDescription)
            } actions: {
                Button("Retour à l'écran de connexion") {
                    session.state = .loggedOut
                }
                    .buttonStyle(.glassProminent)
                    .controlSize(.large)
                    .buttonSizing(.flexible)
            }
        }
    }
}
