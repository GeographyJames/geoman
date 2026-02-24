import { useState } from "react";
import {
  Users,
  UserMinus,
  Plus,
  Pencil,
  Trash2,
  ShieldCheck,
} from "lucide-react";
import { useUsers } from "@/hooks/api/useUsers";
import { useTeams } from "@/hooks/api/useTeams";
import { useBusinessUnits } from "@/hooks/api/useBusinessUnits";
import { usePatchUser } from "@/hooks/api/usePatchUser";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";
import { CreateTeamForm, openCreateTeamModal } from "./CreateTeamForm";
import { EditTeamForm, openEditTeamModal } from "./EditTeamForm";
import { DeleteTeamForm, openDeleteTeamModal } from "./DeleteTeamForm";
import { DeleteUserForm, openDeleteUserModal } from "./DeleteUserForm";
import {
  CreateBusinessUnitForm,
  openCreateBusinessUnitModal,
} from "./CreateBusinessUnitForm";
import {
  EditBusinessUnitForm,
  openEditBusinessUnitModal,
} from "./EditBusinessUnitForm";
import {
  DeleteBusinessUnitForm,
  openDeleteBusinessUnitModal,
} from "./DeleteBusinessUnitForm";
import UserInitials from "@/components/UserInitials";
import type Team from "@/domain/team/entity";
import type User from "@/domain/user/entity";
import type { BusinessUnitOutputDto } from "@/domain/business_unit/outputDto";

function TeamCard({
  team,
  members,
  onEdit,
  onDelete,
  onDeleteUser,
}: {
  team: Team;
  members: User[];
  onEdit: (team: Team) => void;
  onDelete: (team: Team) => void;
  onDeleteUser: (user: User) => void;
}) {
  const { mutate: patchUser, isPending } = usePatchUser();
  const { data: currentUser } = useCurrentUser();
  const isCurrentUserTeam = currentUser?.teamId === team.id;

  return (
    <div
      className={`card border border-base-300 ${isCurrentUserTeam ? "bg-primary/10" : "bg-base-100"}`}
    >
      <div className="card-body gap-3">
        <div className="flex items-center justify-between">
          <h3 className="card-title text-base flex items-center gap-2">
            <Users size={16} />
            {team.name}
            {isCurrentUserTeam && (
              <span className="text-base-content/50 text-sm font-normal">
                (your team)
              </span>
            )}
          </h3>
          {currentUser?.isAdmin && (
            <div className="flex gap-1">
              <button
                type="button"
                className="btn btn-ghost btn-xs"
                title="Edit team"
                onClick={() => {
                  onEdit(team);
                  openEditTeamModal();
                }}
              >
                <Pencil size={14} />
              </button>
              <button
                type="button"
                className={`btn btn-ghost btn-xs ${members.length > 0 ? "text-base-content/30" : "text-error"}`}
                title="Delete team"
                disabled={members.length > 0}
                onClick={() => {
                  onDelete(team);
                  openDeleteTeamModal();
                }}
              >
                <Trash2 size={14} />
              </button>
            </div>
          )}
        </div>

        {members.length === 0 ? (
          <p className="text-sm text-base-content/50">No members</p>
        ) : (
          <ul className="space-y-1">
            {[...members]
              .sort((a, b) => a.lastName.localeCompare(b.lastName))
              .map((member) => (
                <li
                  key={member.id}
                  className="flex items-center justify-between gap-2 hover:bg-base-200 rounded px-1 -mx-1"
                >
                  <div className="flex items-center gap-2">
                    <UserInitials firstName={member.firstName} lastName={member.lastName} />
                    <span className="text-sm">
                      {member.firstName} {member.lastName}
                      {member.isAdmin && (
                        <span className="text-xs text-base-content/50 ml-1">
                          (admin)
                        </span>
                      )}
                    </span>
                  </div>
                  {currentUser?.isAdmin && (
                    <div className="flex gap-1">
                      {currentUser.id !== member.id && (
                        <>
                          <button
                            type="button"
                            className={`btn btn-ghost btn-xs ${member.isAdmin ? "text-primary" : "text-base-content/40"}`}
                            title={
                              member.isAdmin ? "Revoke admin" : "Grant admin"
                            }
                            disabled={isPending}
                            onClick={() =>
                              patchUser({
                                userId: member.id,
                                patch: { admin: !member.isAdmin },
                              })
                            }
                          >
                            <ShieldCheck size={16} />
                          </button>
                          <button
                            type="button"
                            className="btn btn-ghost btn-xs text-error"
                            title="Remove from team"
                            disabled={isPending}
                            onClick={() =>
                              patchUser({
                                userId: member.id,
                                patch: { team_id: -1 },
                              })
                            }
                          >
                            <UserMinus size={16} />
                          </button>
                          <button
                            type="button"
                            className="btn btn-ghost btn-xs text-error"
                            title="Delete user"
                            disabled={isPending}
                            onClick={() => {
                              onDeleteUser(member);
                              openDeleteUserModal();
                            }}
                          >
                            <Trash2 size={16} />
                          </button>
                        </>
                      )}
                    </div>
                  )}
                </li>
              ))}
          </ul>
        )}
      </div>
    </div>
  );
}

function UnassignedUsersCard({
  users,
  teams,
  onDeleteUser,
}: {
  users: User[];
  teams: Team[];
  onDeleteUser: (user: User) => void;
}) {
  const { mutate: patchUser, isPending } = usePatchUser();
  const { data: currentUser } = useCurrentUser();

  return (
    <div className="card border border-base-300 bg-base-100">
      <div className="card-body gap-3">
        <h3 className="card-title text-base flex items-center gap-2">
          <Users size={16} />
          Unassigned Users
        </h3>
        <ul className="space-y-1">
          {[...users]
            .sort((a, b) => a.lastName.localeCompare(b.lastName))
            .map((user) => (
              <li
                key={user.id}
                className="flex items-center justify-between gap-2 hover:bg-base-200 rounded px-1 -mx-1"
              >
                <div className="flex items-center gap-2">
                  <UserInitials firstName={user.firstName} lastName={user.lastName} />
                  <span className="text-sm">
                    {user.firstName} {user.lastName}
                    {user.isAdmin && (
                      <span className="text-xs text-base-content/50 ml-1">
                        (admin)
                      </span>
                    )}
                  </span>
                </div>
                {currentUser?.isAdmin && (
                  <div className="flex items-center gap-1">
                    {currentUser.id !== user.id && (
                      <>
                        <button
                          type="button"
                          className={`btn btn-ghost btn-xs ${user.isAdmin ? "text-primary" : "text-base-content/40"}`}
                          title={user.isAdmin ? "Revoke admin" : "Grant admin"}
                          disabled={isPending}
                          onClick={() =>
                            patchUser({
                              userId: user.id,
                              patch: { admin: !user.isAdmin },
                            })
                          }
                        >
                          <ShieldCheck size={16} />
                        </button>
                        <button
                          type="button"
                          className="btn btn-ghost btn-xs text-error"
                          title="Delete user"
                          disabled={isPending}
                          onClick={() => {
                            onDeleteUser(user);
                            openDeleteUserModal();
                          }}
                        >
                          <Trash2 size={16} />
                        </button>
                      </>
                    )}
                    <select
                      className="select select-xs select-bordered"
                      disabled={isPending}
                      value=""
                      onChange={(e) => {
                        const teamId = Number(e.target.value);
                        if (teamId)
                          patchUser({
                            userId: user.id,
                            patch: { team_id: teamId },
                          });
                      }}
                    >
                      <option value="" disabled>
                        Assign to team…
                      </option>
                      {teams.map((team) => (
                        <option key={team.id} value={team.id}>
                          {team.name}
                        </option>
                      ))}
                    </select>
                  </div>
                )}
              </li>
            ))}
        </ul>
      </div>
    </div>
  );
}

function TeamGroup({
  label,
  teams,
  users,
  onEdit,
  onDelete,
  onDeleteUser,
  businessUnit,
  onEditBU,
  onDeleteBU,
}: {
  label: string;
  teams: Team[];
  users: User[];
  onEdit: (team: Team) => void;
  onDelete: (team: Team) => void;
  onDeleteUser: (user: User) => void;
  businessUnit?: BusinessUnitOutputDto;
  onEditBU?: (bu: BusinessUnitOutputDto) => void;
  onDeleteBU?: (bu: BusinessUnitOutputDto) => void;
}) {
  const { data: currentUser } = useCurrentUser();

  const sortedTeams = [...teams].sort((a, b) => {
    if (a.id === currentUser?.teamId) return -1;
    if (b.id === currentUser?.teamId) return 1;
    return 0;
  });

  return (
    <div className="mb-8">
      <div className="flex items-center gap-2 mb-3">
        <h2 className="text-lg font-semibold text-base-content/80">{label}</h2>
        {businessUnit && currentUser?.isAdmin && onEditBU && onDeleteBU && (
          <div className="flex gap-1">
            <button
              type="button"
              className="btn btn-ghost btn-xs"
              title="Edit business unit"
              onClick={() => {
                onEditBU(businessUnit);
                openEditBusinessUnitModal();
              }}
            >
              <Pencil size={14} />
            </button>
            <button
              type="button"
              className={`btn btn-ghost btn-xs ${teams.length > 0 ? "text-base-content/30" : "text-error"}`}
              title="Delete business unit"
              disabled={teams.length > 0}
              onClick={() => {
                onDeleteBU(businessUnit);
                openDeleteBusinessUnitModal();
              }}
            >
              <Trash2 size={14} />
            </button>
          </div>
        )}
      </div>
      {sortedTeams.length > 0 ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          {sortedTeams.map((team) => (
            <TeamCard
              key={team.id}
              team={team}
              members={users.filter((u) => u.teamId === team.id)}
              onEdit={onEdit}
              onDelete={onDelete}
              onDeleteUser={onDeleteUser}
            />
          ))}
        </div>
      ) : (
        <p className="text-sm text-base-content/50">No teams</p>
      )}
    </div>
  );
}

export default function TeamsSection() {
  const { data: teams = [] } = useTeams();
  const { data: users = [] } = useUsers();
  const { data: businessUnits = [] } = useBusinessUnits();
  const { data: currentUser } = useCurrentUser();

  const [editingTeam, setEditingTeam] = useState<Team | null>(null);
  const [deletingTeam, setDeletingTeam] = useState<Team | null>(null);
  const [editingBU, setEditingBU] = useState<BusinessUnitOutputDto | null>(
    null,
  );
  const [deletingBU, setDeletingBU] = useState<BusinessUnitOutputDto | null>(
    null,
  );
  const [deletingUser, setDeletingUser] = useState<User | null>(null);

  const unassignedTeams = teams.filter((t) => t.businessUnitId === null);
  const unassignedUsers = users.filter((u) => u.teamId === -1);

  return (
    <>
      <CreateTeamForm />
      <CreateBusinessUnitForm />
      <EditTeamForm team={editingTeam} onClose={() => setEditingTeam(null)} />
      <DeleteTeamForm
        team={deletingTeam}
        onClose={() => setDeletingTeam(null)}
      />
      <EditBusinessUnitForm
        businessUnit={editingBU}
        onClose={() => setEditingBU(null)}
      />
      <DeleteBusinessUnitForm
        businessUnit={deletingBU}
        onClose={() => setDeletingBU(null)}
      />
      <DeleteUserForm
        user={deletingUser}
        onClose={() => setDeletingUser(null)}
      />

      <div className="mb-6 flex items-start justify-between">
        <div>
          <h1 className="text-2xl font-semibold mb-1">Teams</h1>
          <p className="text-base-content/70">Teams and their members.</p>
        </div>
        {currentUser?.isAdmin && (
          <div className="flex gap-2">
            <button
              className="btn btn-primary gap-2"
              onClick={openCreateBusinessUnitModal}
            >
              <Plus size={20} />
              New Business Unit
            </button>
            <button
              className="btn btn-primary gap-2"
              onClick={openCreateTeamModal}
            >
              <Plus size={20} />
              New Team
            </button>
          </div>
        )}
      </div>

      {businessUnits.map((bu) => {
        const buTeams = teams.filter((t) => t.businessUnitId === bu.id);
        if (buTeams.length === 0 && !currentUser?.isAdmin) return null;
        return (
          <TeamGroup
            key={bu.id}
            label={bu.name}
            teams={buTeams}
            users={users}
            onEdit={setEditingTeam}
            onDelete={setDeletingTeam}
            onDeleteUser={setDeletingUser}
            businessUnit={bu}
            onEditBU={setEditingBU}
            onDeleteBU={setDeletingBU}
          />
        );
      })}

      {unassignedTeams.length > 0 && (
        <TeamGroup
          label="Unassigned"
          teams={unassignedTeams}
          users={users}
          onEdit={setEditingTeam}
          onDelete={setDeletingTeam}
          onDeleteUser={setDeletingUser}
        />
      )}

      {unassignedUsers.length > 0 && (
        <div className="mb-8">
          <h2 className="text-lg font-semibold mb-3 text-base-content/80">
            Unassigned Users
          </h2>
          <UnassignedUsersCard users={unassignedUsers} teams={teams} onDeleteUser={setDeletingUser} />
        </div>
      )}
    </>
  );
}
