#include "../../../include/TimberWolf/World/EntityComponent.hpp"

tw::EntityComponent::EntityComponent (Entity* entity)
: m_entity(entity) {}

tw::Entity* tw::EntityComponent::getEntity () {

    return m_entity;

}

tw::EntityComponent& tw::EntityComponent::attachToEntity (Entity* entity) {

    m_entity = entity;
    return *this;

}

tw::EntityComponent& tw::EntityComponent::detach () {

    return attachToEntity(nullptr);

}
