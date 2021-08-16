export function getFullKey (domain_object) {
  return fullKey(domain_object.identifier)
}

export function fullKey (identifier) {
  return `${identifier.namespace}.${identifier.key}`
}
