//
//  InstanceSelector.swift
//  Pronote
//
//  Created by Jules on 03/08/2026.
//

import SwiftUI
import PronoteKit

struct InstanceSelector: View {
    @Binding var state: AppState
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
                self.state = .connecting(url: url)
            }
            .buttonStyle(.glassProminent)
            .disabled(url.isEmpty)
            .controlSize(.large)
            .buttonSizing(.flexible)
        }
        .padding()
    }
}
