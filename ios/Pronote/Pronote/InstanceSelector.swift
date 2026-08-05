//
//  InstanceSelector.swift
//  Pronote
//
//  Created by Jules on 03/08/2026.
//

import SwiftUI
import PronoteKit

struct InstanceSelector: View {
    @Binding var instance: Instance?
    @Binding var state: AppState
    @State private var url: String = ""
    @State private var loading: Bool = false
    
    var body: some View {
        if loading == false {
            VStack {
                HStack {
                    Image(systemName: "link")
                    TextField("URL de l'instance PRONOTE", text: $url)
                        .autocorrectionDisabled(true)
                        .textInputAutocapitalization(.never)
                        .keyboardType(.URL)
                        .textFieldStyle(.plain)
                }
                .padding()
                .glassEffect()
                Button {
                    Task {
                        loading = true
                        instance = try! await Instance(url: url)
                        state = .authentication
                    }
                } label: {
                    Text("Suivant")
                }
                .disabled(url.isEmpty)
                .buttonStyle(.glassProminent)
            }
            .padding()
        } else {
            ProgressView("Chargement de l'instance PRONOTE")
        }
    }
}

#Preview {
    InstanceSelector(instance: .constant(nil), state: .constant(.instanceSelection))
}
