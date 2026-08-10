//
//  LoadableView.swift
//  Libellule
//
//  Created by Jules on 10/08/2026.
//

import SwiftUI

struct LoadableView<T, Content: View>: View {
    let state: Loadable<T>
    let retry: () async -> Void
    @ViewBuilder let content: (T) -> Content
    
    var body: some View {
        switch state {
        case .idle, .loading:
            ProgressView()
        case .failed(let error):
            ContentUnavailableView {
                Label("Erreur", systemImage: "wifi.exclamationmark")
            } description: {
                Text(error.localizedDescription)
            } actions: {
                Button("Réessayer") { Task { await retry() }}
            }
        case .loaded(let value):
            content(value)
        }
    }
}
