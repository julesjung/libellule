//
//  ContentView.swift
//  Pronote
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI
import PronoteKit

struct ContentView: View {
    @State private var client: Client?
    @State private var parameters: ParametersRecord?
    @State private var username: String = ""
    @State private var password: String = ""
    
    var body: some View {
        VStack {
            Spacer()
            TextField("Username", text: $username)
                .autocorrectionDisabled(true)
                .textInputAutocapitalization(.never)
                .textFieldStyle(.roundedBorder)
            SecureField("Password", text: $password)
                .textFieldStyle(.roundedBorder)
            Button {
                Task {
                    client = try! await Client(instanceUrl: "https://demo.index-education.net/pronote/")
                    
                    parameters = try! await client?.connect()
                }
            } label: {
                Text("Log In")
            }
            .disabled(true)
            .buttonSizing(.flexible)
            .buttonStyle(.glassProminent)
            Spacer()
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
