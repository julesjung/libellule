//
//  InstanceView.swift
//  Libellule
//
//  Created by Jules on 03/08/2026.
//

import SwiftUI
import LibelluleKit

struct InstanceView: View {
    @Environment(SessionStore.self) private var session
    @State private var url: String = ""

    var body: some View {
        VStack {
            HStack {
                Image(systemName: "link")
                TextField("URL de l'instance PRONOTE", text: $url)
                    .autocorrectionDisabled(true)
                    .textInputAutocapitalization(.never)
                    .keyboardType(.URL)
            }
            .padding()
            .textFieldStyle(.plain)
            .glassEffect()

            Button("Suivant") {
                Task {
                    await session.connect(url: url)
                }
            }
            .buttonStyle(.glassProminent)
            .disabled(url.isEmpty)
            .controlSize(.large)
            .buttonSizing(.flexible)
        }
        .padding()
    }
}
